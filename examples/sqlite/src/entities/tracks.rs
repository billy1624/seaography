use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "tracks")]
#[graphql(complex)]
#[graphql(name = "Tracks")]
pub struct Model {
    #[sea_orm(column_name = "TrackId", primary_key)]
    pub track_id: i32,
    #[sea_orm(column_name = "Name")]
    pub name: String,
    #[sea_orm(column_name = "AlbumId")]
    pub album_id: Option<i32>,
    #[sea_orm(column_name = "MediaTypeId")]
    pub media_type_id: i32,
    #[sea_orm(column_name = "GenreId")]
    pub genre_id: Option<i32>,
    #[sea_orm(column_name = "Composer")]
    pub composer: Option<String>,
    #[sea_orm(column_name = "Milliseconds")]
    pub milliseconds: i32,
    #[sea_orm(column_name = "Bytes")]
    pub bytes: Option<i32>,
    #[sea_orm(column_name = "UnitPrice")]
    pub unit_price: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::media_types::Entity",
        from = "Column::MediaTypeId",
        to = "super::media_types::Column::MediaTypeId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    MediaTypes,
    #[sea_orm(
        belongs_to = "super::genres::Entity",
        from = "Column::GenreId",
        to = "super::genres::Column::GenreId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Genres,
    #[sea_orm(
        belongs_to = "super::albums::Entity",
        from = "Column::AlbumId",
        to = "super::albums::Column::AlbumId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Albums,
    #[sea_orm(has_many = "super::invoice_items::Entity")]
    InvoiceItems,
    #[sea_orm(has_many = "super::playlist_track::Entity")]
    PlaylistTrack,
}

impl Related<super::media_types::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MediaTypes.def()
    }
}

impl Related<super::genres::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Genres.def()
    }
}

impl Related<super::albums::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Albums.def()
    }
}

impl Related<super::invoice_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InvoiceItems.def()
    }
}

impl Related<super::playlist_track::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlaylistTrack.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
