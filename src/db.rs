use crate::models::{PgView, PgViewColumn};
use crate::CliArgs;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};
use std::collections::HashMap;

pub async fn init_db(connection_string: String) -> Result<Pool<Postgres>, Error> {
    let connection_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string.as_str())
        .await;

    return match connection_pool {
        Ok(connection_pool) => Ok(connection_pool),
        Err(err) => Err(err),
    };
}

pub fn get_connection_string_from_args(args: &CliArgs) -> String {
    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        args.user, args.password, args.host, args.database_port, args.database_name
    );
    println!(
        "postgres://{}:{}@{}:{}/{}",
        args.user, args.password, args.host, args.database_port, args.database_name
    );

    return connection_string;
}

pub async fn get_view_rows(schemas: &String, pool: &Pool<Postgres>) -> HashMap<String, String> {
    let view_rows = sqlx::query_as::<_, PgView>(
        "
                        select table_name,
                            view_definition
                            from information_schema.views where table_schema in ($1)
                            order by table_name",

    ).bind(schemas)
    .fetch_all(pool)
    .await.unwrap();

    let mut view_rows_map = HashMap::new();
    for row in view_rows {
        view_rows_map.insert(row.table_name, row.view_definition);
    }

    view_rows_map
}

pub async fn get_view_column_rows(schemas: &String, pool: &Pool<Postgres>) -> HashMap<String, String> {
    let view_column_rows = sqlx::query_as::<_, PgViewColumn>("
                        select
                            table_name,
                            array_agg(column_name || '?: ' || data_type || ';')::text as columns
                        from information_schema.columns where table_name in (
                            select views.table_name from information_schema.views where table_schema in ($1)
                        )
                        group by table_name
                        ",)
        .bind(schemas)
        .fetch_all(pool)
        .await.unwrap();

    let mut view_column_rows_map = HashMap::new();

    for row in view_column_rows {
        view_column_rows_map.insert(row.table_name, row.columns);
    }

    view_column_rows_map
}
