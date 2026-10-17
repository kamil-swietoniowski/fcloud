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
    id: Uuid,
    name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
    email_address: Option<String>,
    description: Option<String>
}

#[derive(sqlx::FromRow, Debug)]
pub struct Place {
    id: Uuid,
    display_name: Option<String>,
    geo_coords: Option<String>,
    description: Option<String>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Note {
    id: Uuid,
    title: Option<String>,
    content: Option<String>,
    edited: DateTime<Utc>
}

#[derive(sqlx::FromRow, Debug)]
pub struct Relation {
    id: Uuid,
    id_1: Uuid,
    id_2: Uuid,
    id_1_table: Table,
    id_2_table: Table
}

impl Person {
    pub fn new(name: Option<String>, last_name: Option<String>, phone_number: Option<String>, email_address: Option<String>, description: Option<String>) -> Self {
        let uuid = Uuid::now_v7();

        Self {
            id: uuid, name, last_name, phone_number, email_address, description
        }
    }
}
