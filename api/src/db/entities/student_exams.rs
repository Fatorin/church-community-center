//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "student_exams")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub student_infos_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub semester: i16,
    #[sea_orm(primary_key, auto_increment = false)]
    pub exam_type: i16,
    pub chinese_score: Option<i16>,
    pub english_score: Option<i16>,
    pub math_score: Option<i16>,
    pub science_score: Option<i16>,
    pub social_studies_score: Option<i16>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::student_infos::Entity",
        from = "Column::StudentInfosId",
        to = "super::student_infos::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    StudentInfos,
}

impl Related<super::student_infos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StudentInfos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
