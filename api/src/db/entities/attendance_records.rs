//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "attendance_records")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    pub date: Date,
    #[sea_orm(column_type = "Text", nullable)]
    pub note: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::attendance_students::Entity")]
    AttendanceStudents,
}

impl Related<super::attendance_students::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AttendanceStudents.def()
    }
}

impl Related<super::students::Entity> for Entity {
    fn to() -> RelationDef {
        super::attendance_students::Relation::Students.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::attendance_students::Relation::AttendanceRecords
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
