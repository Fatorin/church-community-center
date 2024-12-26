//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "student_grades")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub student_id: Uuid,
    pub academic_year: i16,
    pub semester: i16,
    pub exam_type: i16,
    pub chinese_score: Option<i16>,
    pub english_score: Option<i16>,
    pub math_score: Option<i16>,
    pub science_score: Option<i16>,
    pub social_studies_score: Option<i16>,
    #[sea_orm(column_type = "Text", nullable)]
    pub comment: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::students::Entity",
        from = "Column::StudentId",
        to = "super::students::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Students,
}

impl Related<super::students::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Students.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
