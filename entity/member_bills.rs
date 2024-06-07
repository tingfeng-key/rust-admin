//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "member_bills")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub member_id: i32,
    pub r#type: i32,
    pub pm: i32,
    pub number: Decimal,
    #[sea_orm(column_type = "custom(\"DATETIME\")")]
    pub created_at: String,
    #[sea_orm(column_type = "custom(\"DATETIME\")")]
    pub updated_at: String,
    #[sea_orm(column_type = "custom(\"DATETIME\")", nullable)]
    pub deleted_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::members::Entity",
        from = "Column::MemberId",
        to = "super::members::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Members,
}

impl Related<super::members::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Members.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
