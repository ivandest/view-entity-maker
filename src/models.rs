#[derive(sqlx::FromRow)]
pub struct PgView {
    pub table_name: String,
    pub view_definition: String,
}

#[derive(sqlx::FromRow)]
pub struct PgViewColumn {
    pub table_name: String,
    pub columns: String,
}
