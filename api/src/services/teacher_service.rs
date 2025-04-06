use crate::db::entities::{members, teachers};
use crate::models::{
    teacher_and_member_to_view, AddTeacherRequest, AppResponse, EmploymentType, MemberDto,
    RoleType, TeacherView, UpdateTeacherRequest,
};
use crate::services::prelude::*;
use crate::util::Claims;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait,
};
use uuid::Uuid;
use validator::Validate;

pub async fn add_teacher(
    Extension(claims): Extension<Claims>,
    State(db): State<DatabaseConnection>,
    Json(payload): Json<AddTeacherRequest>,
) -> Result<Json<AppResponse>, (StatusCode, Json<AppResponse>)> {
    if let Err(err) = payload.validate() {
        return Err(AppResponse::error(
            StatusCode::BAD_REQUEST,
            format!("參數不正確。錯誤參數：{}", err),
        ));
    }

    check_permission(claims.role)?;

    if teachers::Entity::find()
        .filter(teachers::Column::Username.eq(&payload.username))
        .one(&db)
        .await
        .map_err(|_| AppResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "伺服器發生異常"))?
        .is_some()
    {
        return Err(AppResponse::error(
            StatusCode::BAD_REQUEST,
            "此帳號已被註冊",
        ));
    }

    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|_| AppResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "伺服器發生異常"))?;

    let new_teacher = teachers::ActiveModel {
        username: Set(payload.username),
        password: Set(password_hash),
        role_type: Set(RoleType::Admin.into()),
        employment_type: Set(EmploymentType::FullTime.into()),
        responsibility: Set(payload.responsibility),
        background: Set(payload.background),
        ..Default::default()
    };

    new_teacher
        .insert(&db)
        .await
        .map_err(|_| AppResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "伺服器發生異常"))?;

    Ok(AppResponse::success("建立成功"))
}

pub async fn get_teachers(
    State(db): State<DatabaseConnection>,
) -> Result<Json<AppResponse<Vec<TeacherView>>>, (StatusCode, Json<AppResponse>)> {
    let teachers_with_members = teachers::Entity::find()
        .filter(teachers::Column::DeletedAt.is_null())
        .find_with_related(members::Entity)
        .all(&db)
        .await
        .map_err(|_| AppResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "資料庫異常"))?;

    let mut result = vec![];
    for (teacher, mut members) in teachers_with_members {
        if let Some(member) = members.pop() {
            let view = teacher_and_member_to_view(teacher, member);
            result.push(view);
        } else {
            return Err(AppResponse::error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Teacher without member",
            ));
        }
    }

    Ok(AppResponse::success_with_data(result))
}

pub async fn update_teacher(
    Extension(claims): Extension<Claims>,
    State(db): State<DatabaseConnection>,
    Path(teacher_id): Path<Uuid>,
    Json(payload): Json<UpdateTeacherRequest>,
) -> Result<Json<AppResponse<TeacherView>>, (StatusCode, Json<AppResponse>)> {
    if let Err(err) = payload.validate() {
        return Err(AppResponse::error(
            StatusCode::BAD_REQUEST,
            format!("參數不正確。錯誤參數：{}", err),
        ));
    }

    if claims.sub != teacher_id || claims.role != RoleType::SuperAdmin {
        return Err(AppResponse::error(
            StatusCode::FORBIDDEN,
            "非本人或是系統管理員，無法修改。",
        ));
    }

    let txn = db
        .begin()
        .await
        .map_err(|_| AppResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "資料庫異常"))?;

    let option_member = find_member_by_id(&db, teacher_id.clone()).await?;
    let member = update_member_with_context(&txn, option_member, payload.member_dto).await?;

    let teacher = match find_teacher_by_id(&db, teacher_id).await? {
        Some(teacher) => {
            let mut teacher: teachers::ActiveModel = teacher.into();

            if let Some(password) = payload.password {
                let password_hash = hash(password, DEFAULT_COST).map_err(|_| {
                    AppResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "伺服器發生異常")
                })?;
                teacher.password = Set(password_hash);
            }

            teacher.employment_type = Set(payload.employment_type.into());
            teacher.responsibility = Set(payload.responsibility);
            teacher.background = Set(payload.background);
            teacher.updated_at = Set(Utc::now().naive_utc());

            let teacher: teachers::Model = teacher
                .update(&db)
                .await
                .map_err(|_| AppResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "更新失敗"))?;

            teacher
        }
        None => {
            return Err(AppResponse::error(
                StatusCode::BAD_REQUEST,
                "找不到對應的教職員",
            ));
        }
    };

    let teacher_view = TeacherView {
        member_id: member.id.clone(),
        username: teacher.username,
        employment_type: teacher.employment_type.into(),
        responsibility: teacher.responsibility,
        background: teacher.background,
        member_dto: MemberDto::from(member),
    };

    Ok(AppResponse::success_with_data(teacher_view))
}

pub async fn delete_teacher(
    Extension(claims): Extension<Claims>,
    State(db): State<DatabaseConnection>,
    Path(teacher_id): Path<Uuid>,
) -> Result<Json<AppResponse>, (StatusCode, Json<AppResponse>)> {
    check_permission(claims.role)?;

    if let Some(teacher) = find_teacher_by_id(&db, teacher_id).await? {
        let mut teacher: teachers::ActiveModel = teacher.into();
        teacher.updated_at = Set(Utc::now().naive_utc());
        teacher.deleted_at = Set(Some(Utc::now().naive_utc()));

        teacher
            .update(&db)
            .await
            .map_err(|_| AppResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "更新失敗"))?;

        Ok(AppResponse::success("刪除成功"))
    } else {
        Err(AppResponse::error(
            StatusCode::BAD_REQUEST,
            "找不到對應的教職員",
        ))
    }
}

async fn find_teacher_by_id(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<Option<teachers::Model>, (StatusCode, Json<AppResponse>)> {
    teachers::Entity::find()
        .filter(teachers::Column::MemberId.eq(id))
        .filter(teachers::Column::DeletedAt.is_null())
        .one(db)
        .await
        .map_err(|_| AppResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "資料庫異常"))
}

fn check_permission(role_type: RoleType) -> Result<(), (StatusCode, Json<AppResponse>)> {
    if role_type.ne(&RoleType::SuperAdmin) {
        return Err(AppResponse::error(
            StatusCode::FORBIDDEN,
            "主管理員才可使用",
        ));
    }

    Ok(())
}
