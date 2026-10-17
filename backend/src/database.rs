use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::str::FromStr;

const SQL_START_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS People (
        id TEXT PRIMARY KEY NOT NULL,
        name TEXT,
        last_name TEXT,
        phone_number TEXT,
        email_address TEXT,
        description TEXT
    );
    
    CREATE TABLE IF NOT EXISTS Places (
        id TEXT PRIMARY KEY NOT NULL,
        display_name TEXT,
        geo_coords TEXT,
        description TEXT
    );

    CREATE TABLE IF NOT EXISTS Notes (
        id TEXT PRIMARY KEY NOT NULL,
        name TEXT,
        title TEXT NOT NULL,
        content TEXT,
        edited TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS Relation (
        id TEXT PRIMARY KEY NOT NULL,
        id_1 TEXT NOT NULL,
        id_2 TEXT NOT NULL,
        id_1_table TEXT NOT NULL,
        id_2_table TEXT NOT NULL
    );
"#;


pub async fn connect_to_db() -> Result<Pool<Sqlite>, Box<dyn std::error::Error>> {
    let connection_options = SqliteConnectOptions::from_str("sqlite://baza.db")?
        .create_if_missing(true);

    let pool: Pool<Sqlite> = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await?;

    sqlx::query(SQL_START_QUERY)
        .execute(&pool)
        .await?;
    Ok(pool)
}

use crate::models::Person;
pub async fn insert_person(pool: &Pool<Sqlite>, person: &Person) -> Result<(), sqlx::Error> { 
    sqlx::query!(
        "INSERT INTO people (id, name, last_name, phone_number, email_address, description)
        VALUES (?, ?, ?, ?, ?, ?)",
        person.id,
        person.name,
        person.last_name,
        person.phone_number,
        person.email_address,
        person.description


    )
    Ok(())
}
pub async fn get_person(pool: &Pool<Sqlite>, id: &str) -> Result<Person, sqlx::Error> { 
    let person = Person::new(None, None, None, None, None);
    Ok(person)
 }
