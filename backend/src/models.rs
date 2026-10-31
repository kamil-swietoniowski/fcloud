#![allow(dead_code)]

use std::os::unix::net::UnixListener;

use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

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


impl Person {
    pub fn new(id: Option<Uuid>, name: Option<String>, last_name: Option<String>, phone_number: Option<String>, email_address: Option<String>, description: Option<String>) -> Self {
        let uuid = Uuid::now_v7();

        Self {
            id: uuid, name, last_name, phone_number, email_address, description
        }
    }
}

impl Place {
    pub fn new(id: Option<Uuid>, display_name: Option<String>, geo_coords: Option<String>, description: Option<String>) -> Self {
        let uuid = Uuid::now_v7();

        Self {
            id: uuid, display_name, geo_coords, description
        }
    }
}

impl Note {
    pub fn new(id: Option<Uuid>, title: Option<String>, content: Option<String>, edited: Option<DateTime<Utc>>) -> Self {
        let id = if id.is_none() {
            Uuid::now_v7()
        } else {
            id.unwrap()
        };

        let edited = if edited.is_none() {
            Utc::now()
        } else {
            edited.unwrap()
        };

        Self {
            id, title, content, edited
        }
    }
}