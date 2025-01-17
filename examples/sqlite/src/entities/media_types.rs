use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "media_types")]
#[graphql(complex)]
#[graphql(name = "MediaTypes")]
pub struct Model {
    #[sea_orm(column_name = "MediaTypeId", primary_key)]
    pub media_type_id: i32,
    #[sea_orm(column_name = "Name")]
    pub name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(has_many = "super::tracks::Entity")]
    Tracks,
}

impl Related<super::tracks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tracks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
