use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "playlist_track")]
#[graphql(complex)]
#[graphql(name = "PlaylistTrack")]
pub struct Model {
    #[sea_orm(column_name = "PlaylistId", primary_key, auto_increment = false)]
    pub playlist_id: i32,
    #[sea_orm(column_name = "TrackId", primary_key, auto_increment = false)]
    pub track_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tracks::Entity",
        from = "Column::TrackId",
        to = "super::tracks::Column::TrackId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Tracks,
    #[sea_orm(
        belongs_to = "super::playlists::Entity",
        from = "Column::PlaylistId",
        to = "super::playlists::Column::PlaylistId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Playlists,
}

impl Related<super::tracks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tracks.def()
    }
}

impl Related<super::playlists::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Playlists.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
