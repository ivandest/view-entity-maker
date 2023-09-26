extern crate core;

use crate::db::{get_view_column_rows, get_view_rows};
use clap::Parser;
use sqlx::Error;

mod db;
mod generate;
mod models;

#[derive(Parser)]
pub struct CliArgs {
    host: String,
    database_port: String,
    database_name: String,
    user: String,
    password: String,
    source_schemas: Option<String>,
    exclude_schemas: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: CliArgs = CliArgs::parse();
    let pool = db::init_db(db::get_connection_string_from_args(&args)).await;

    match pool {
        Ok(pool) => {
            let schemas = &args.source_schemas;

            match schemas {
                Some(schemas) => {
                    let view_rows = get_view_rows(schemas, &pool).await;

                    let view_column_rows = get_view_column_rows(schemas, &pool).await;

                    for (key, value) in view_rows.iter() {
                        println!("RESULT {} --> {}", key, value);
                    }

                    for (key, value) in view_column_rows.iter() {
                        println!("RESULT {} --> {}", key, value,);
                    }

                    generate::generate_file(view_rows, view_column_rows).expect("File generation failed");
                }
                None => println!("No schemas in args!")
            }

            Ok(())
        }
        Err(error) => Err(error),
    }
}
