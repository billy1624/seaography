use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "customers")]
#[graphql(complex)]
#[graphql(name = "Customers")]
pub struct Model {
    #[sea_orm(column_name = "CustomerId", primary_key)]
    pub customer_id: i32,
    #[sea_orm(column_name = "FirstName")]
    pub first_name: String,
    #[sea_orm(column_name = "LastName")]
    pub last_name: String,
    #[sea_orm(column_name = "Company")]
    pub company: Option<String>,
    #[sea_orm(column_name = "Address")]
    pub address: Option<String>,
    #[sea_orm(column_name = "City")]
    pub city: Option<String>,
    #[sea_orm(column_name = "State")]
    pub state: Option<String>,
    #[sea_orm(column_name = "Country")]
    pub country: Option<String>,
    #[sea_orm(column_name = "PostalCode")]
    pub postal_code: Option<String>,
    #[sea_orm(column_name = "Phone")]
    pub phone: Option<String>,
    #[sea_orm(column_name = "Fax")]
    pub fax: Option<String>,
    #[sea_orm(column_name = "Email")]
    pub email: String,
    #[sea_orm(column_name = "SupportRepId")]
    pub support_rep_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::employees::Entity",
        from = "Column::SupportRepId",
        to = "super::employees::Column::EmployeeId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Employees,
    #[sea_orm(has_many = "super::invoices::Entity")]
    Invoices,
}

impl Related<super::employees::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employees.def()
    }
}

impl Related<super::invoices::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Invoices.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
