use std::collections::HashMap;
use postgres::{Client, NoTls};
use crate::error::error::{ApplicationError, DatabaseError, Error};
use crate::models::{Readable, Writable};

pub const DB_HOST: &str = "localhost";
pub const DB_USER: &str = "postgres";
pub const DB_NAME: &str = "guestbook";

pub fn create_and_use_database() -> Result<(), Error> {
    let mut client = Client::connect(&format!("host={DB_HOST} user={DB_USER} dbname={DB_NAME}"), NoTls)?;

    client.batch_execute(&format!("CREATE DATABASE {DB_NAME}"))?;
    client.batch_execute(&format!("SET search_path = {DB_NAME}"))?;

    Ok(())
}

pub fn create_table(table_name: &str, attributes: HashMap<&str, &str>) -> Result<(), Error>  {
    let mut client = Client::connect(&format!("host={DB_HOST} user={DB_USER} dbname={DB_NAME}"), NoTls)?;
    let create_table_op = format!("CREATE TABLE IF NOT EXISTS {table_name} (");

    let mut attributes_str = String::new();
    for i in attributes.iter() {
        attributes_str += &((*i.0).to_owned() + " " + *i.1 + ",");
    }
    attributes_str.pop();

    let command = create_table_op + &attributes_str + ")";
    client.batch_execute(&command)?;

    Ok(())
}

pub fn write_to_database(writable: &dyn Writable) -> Result<(), Error>  {
    let mut client = Client::connect(&format!("host={DB_HOST} user={DB_USER} dbname={DB_NAME}"), NoTls)?;
    writable.save_to_database(&mut client).unwrap();

    Ok(())
}

pub fn read_from_database<T: Readable>(table_name: &str) -> Result<Vec<T>, Error> {
    let mut client = Client::connect(&format!("host={DB_HOST} user={DB_USER} dbname={DB_NAME}"), NoTls)?;
    let rows = client.query(&format!("SELECT * FROM {}", table_name), &[])?;
    let mut results: Vec<T> = Vec::new();

    for row in rows {
        results.push(T::from_row(&row)?);
    }

    Ok(results)
}

// todo: use hashmap instead of two vectors
pub fn get_record_id_from_info(id_name: &str, table_name: &str, attributes: &Vec<&str>, info: &Vec<&str>) -> Result<i32, Error> {
    let mut client = Client::connect(&format!("host={DB_HOST} user={DB_USER} dbname={DB_NAME}"), NoTls)?;
    let query = format!(
        "SELECT {} FROM {} WHERE {}",
        id_name,
        table_name,
        info.iter()
            .enumerate()
            .map(|(i, _)| format!("{} = ${}", attributes[i], i + 1))
            .collect::<Vec<_>>()
            .join(" AND ")
    );

    let params: Vec<String> = info.iter().map(|s| format!("{}", s)).collect();
    let params_ref: Vec<&(dyn postgres::types::ToSql + Sync)> = params.iter().map(|s| s as &(dyn postgres::types::ToSql + Sync)).collect();
    let row = client.query(&query, &params_ref)?;

    if row.is_empty() {
        return Err(Error::new(ApplicationError::DatabaseError(DatabaseError::Error("Error getting id from info".to_string())), "row is empty".to_string()));
    }

    let record_id: i32 = row[0].get(id_name);
    Ok(record_id)
}