//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "country"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub country_id: u16,
    pub country: String,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    CountryId,
    Country,
    LastUpdate,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    CountryId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = u16;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    City,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::CountryId => ColumnType::SmallUnsigned.def(),
            Self::Country => ColumnType::String(Some(50u32)).def(),
            Self::LastUpdate => ColumnType::Timestamp.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::City => Entity::has_many(super::city::Entity).into(),
        }
    }
}

impl Related<super::city::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::City.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}