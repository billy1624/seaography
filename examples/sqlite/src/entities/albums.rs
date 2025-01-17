use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "albums")]
#[graphql(complex)]
#[graphql(name = "Albums")]
pub struct Model {
    #[sea_orm(column_name = "AlbumId", primary_key)]
    pub album_id: i32,
    #[sea_orm(column_name = "Title")]
    pub title: String,
    #[sea_orm(column_name = "ArtistId")]
    pub artist_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::artists::Entity",
        from = "Column::ArtistId",
        to = "super::artists::Column::ArtistId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Artists,
    #[sea_orm(has_many = "super::tracks::Entity")]
    Tracks,
}

impl Related<super::artists::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artists.def()
    }
}

impl Related<super::tracks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tracks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
