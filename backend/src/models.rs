#![allow(dead_code)]

use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, sqlx::Type, Debug, Clone, PartialEq)]
#[sqlx(type_name = "TEXT")]
enum Table {
    Person,
    Place,
    Note, 
}

#[derive(sqlx::FromRow, Debug)]
pub struct Person {
    pub id: Uuid,
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub email_address: Option<String>,
    pub description: Option<String>
}

#[derive(sqlx::FromRow, Debug)]
pub struct Place {
    pub id: Uuid,
    pub display_name: Option<String>,
    pub geo_coords: Option<String>,
    pub description: Option<String>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Note {
    pub id: Uuid,
    pub title: Option<String>,
    pub content: Option<String>,
    pub edited: DateTime<Utc>
}

#[derive(sqlx::FromRow, Debug)]
pub struct Relation {
    pub id: Uuid,
    pub id_1: Uuid,
    pub id_2: Uuid,
    pub id_1_table: Table,
    pub id_2_table: Table
}

impl Person {
    pub fn new(name: Option<String>, last_name: Option<String>, phone_number: Option<String>, email_address: Option<String>, description: Option<String>) -> Self {
        let uuid = Uuid::now_v7();

        Self {
            id: uuid, name, last_name, phone_number, email_address, description
        }
    }
}
