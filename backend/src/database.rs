use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::str::FromStr;
use uuid::Uuid;

const SQL_START_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS people (
        id TEXT PRIMARY KEY NOT NULL,
        name TEXT,
        last_name TEXT,
        phone_number TEXT,
        email_address TEXT,
        description TEXT
    );
    
    CREATE TABLE IF NOT EXISTS places (
        id TEXT PRIMARY KEY NOT NULL,
        display_name TEXT,
        geo_coords TEXT,
        description TEXT
    );

    CREATE TABLE IF NOT EXISTS notes (
        id TEXT PRIMARY KEY NOT NULL,
        name TEXT,
        title TEXT NOT NULL,
        content TEXT,
        edited TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS relation (
        id TEXT PRIMARY KEY NOT NULL,
        id_1 TEXT NOT NULL,
        id_2 TEXT NOT NULL,
        id_1_table TEXT NOT NULL,
        id_2_table TEXT NOT NULL
    );
"#;


pub async fn connect_to_db(path: &str) -> Result<Pool<Sqlite>, Box<dyn std::error::Error>> {
    let connection_options = SqliteConnectOptions::from_str(path)?
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
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn get_person(pool: &Pool<Sqlite>, id: &str) -> Result<Person, sqlx::Error> { 
    let person_uuid = uuid::Uuid::parse_str(id)
        .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

    
    let person = sqlx::query_as!(
        Person,
        r#"
        SELECT 
            id as "id: uuid::Uuid", 
            name as "name?", 
            last_name as "last_name?", 
            phone_number as "phone_number?", 
            email_address as "email_address?", 
            description as "description?"
        FROM people 
        WHERE id = ?
        "#,
        person_uuid
    )
    .fetch_one(pool)
    .await?;

    Ok(person)
 }

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn person_crud() -> Result<(), Box<dyn std::error::Error>> {
        let db = connect_to_db("sqlite::memory:").await?;
        let person1 = Person::new(
            Some("Adam".into()), 
            None, 
            None,
            None,
            None
        );
        let uuid = person1.id.to_string();
        insert_person(&db, &person1).await?;

        let person2= get_person(&db, &uuid).await?;
        assert_eq!(person1.name, person2.name);

        Ok(())
    }
}
