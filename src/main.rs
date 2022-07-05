extern crate core;

use crate::db::{get_view_column_rows, get_view_rows};
use crate::models::{PgView, PgViewColumn};
use clap::Parser;
use sqlx::Error;
use std::collections::HashMap;

mod db;
mod generate;
mod models;

#[derive(Parser)]
pub struct CliArgs {
    host: String,
    user: String,
    password: String,
    database_name: String,
    database_port: String,
    source_schemas: Option<String>,
    exclude_schemas: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: CliArgs = CliArgs::parse();
    let pool = db::init_db(db::get_connection_string_from_args(args)).await;

    match pool {
        Ok(pool) => {
            let schemas = &args.source_schemas;

            let view_rows = get_view_rows(schemas.unwrap(), pool);

            let view_column_rows = get_view_column_rows(schemas.unwrap(), pool);

            for (key, value) in view_rows.iter() {
                println!("RESULT {} --> {}", key, value);
            }

            for (key, value) in view_column_rows.iter() {
                println!("RESULT {} --> {}", key, value,);
            }

            generate::generate_file(view_rows, view_column_rows).expect("File generation failed");

            Ok(())
        }
        Err(error) => Err(error),
    }
}
