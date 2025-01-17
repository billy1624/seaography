use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "employees")]
#[graphql(complex)]
#[graphql(name = "Employees")]
pub struct Model {
    #[sea_orm(column_name = "EmployeeId", primary_key)]
    pub employee_id: i32,
    #[sea_orm(column_name = "LastName")]
    pub last_name: String,
    #[sea_orm(column_name = "FirstName")]
    pub first_name: String,
    #[sea_orm(column_name = "Title")]
    pub title: Option<String>,
    #[sea_orm(column_name = "ReportsTo")]
    pub reports_to: Option<i32>,
    #[sea_orm(column_name = "BirthDate")]
    pub birth_date: Option<DateTime>,
    #[sea_orm(column_name = "HireDate")]
    pub hire_date: Option<DateTime>,
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
    pub email: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ReportsTo",
        to = "Column::EmployeeId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SelfRef,
    #[sea_orm(has_many = "super::customers::Entity")]
    Customers,
}

impl Related<super::customers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
