use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "playlists")]
#[graphql(complex)]
#[graphql(name = "Playlists")]
pub struct Model {
    #[sea_orm(column_name = "PlaylistId", primary_key)]
    pub playlist_id: i32,
    #[sea_orm(column_name = "Name")]
    pub name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(has_many = "super::playlist_track::Entity")]
    PlaylistTrack,
}

impl Related<super::playlist_track::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlaylistTrack.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
