use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{Error, Write};

mod utils;

pub fn generate_file(
    view_rows_map: HashMap<String, String>,
    view_column_rows_map: HashMap<String, String>,
) -> Result<(), Error> {
    for (key, value) in view_rows_map.iter() {
        let expr = value.to_string();
        let view_name = key;

        let columns: &String = view_column_rows_map.get(key).expect("aaaaa");
        let mut content = String::new();

        let column_array_string = &columns
            .replace("{", "")
            .replace("}", "")
            .replace("\"", "")
            .replace(",", "");

        let column_array: Vec<&str> = column_array_string.split(";").collect();

        let mut columns_output = String::new();
        for column in column_array {
            let column_parts: Vec<&str> = column.split(":").collect();
            if column_parts.len() < 2 {
                continue;
            }
            let decorator = "@ViewColumn()\n";

            columns_output.push_str(decorator);

            columns_output.push_str(parse_column(column_parts).as_str());
        }

        write!(
            &mut content,
            "import {{ ViewColumn, ViewEntity }} from 'typeorm';

@ViewEntity({{
    schema: 'analytics',
    expression: `{}`
}})
export class {} {{
{}
}}",
            expr,
            utils::make_camel_case(view_name.to_string(), true),
            columns_output
        )
        .expect("TODO: panic message");
        let path = format!("{}.view.ts", utils::make_kebab_case(view_name.to_string()));

        let mut file = File::create(path)?;
        file.write_all(content.as_ref())?;
    }

    Ok(())
}

fn parse_column(parts: Vec<&str>) -> String {
    let mut output = String::new();
    let column_name = parts[0].trim().to_string();
    let mut column_type = parts[1].to_string();

    if column_type.contains("integer") {
        column_type = column_type.replace("integer", "number");
    }
    if column_type.contains("numeric") {
        column_type = column_type.replace("numeric", "number");
    }

    if column_type.contains("timestamp with time zone") {
        column_type = column_type.replace("timestamp with time zone", "string");
    }
    if column_type.contains("character varying") {
        column_type = column_type.replace("character varying", "string");
    }
    if column_type.contains("text") {
        column_type = column_type.replace("text", "string");
    }

    output.push_str(&*utils::make_camel_case(column_name, false));
    output.push_str((":".to_string() + column_type.as_str() + ";\n\n").as_str());

    output
}
