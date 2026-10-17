use uuid::{uuid, Uuid};
use chrono::{DateTime, Utc};


enum Table {
    Person,
    Place,
    Note, 
}


pub struct Person {
    id: Uuid,
    name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
    email_address: Option<String>,
    description: Option<String>
}

pub struct Place {
    id: Uuid,
    display_name: Option<String>,
    geo_coords: Option<String>,
    description: Option<String>,
}

pub struct Note {
    id: Uuid,
    title: Option<String>,
    content: Option<String>,
    edited: DateTime<Utc>
}

pub struct Relation {
    id: Uuid,
    id_1: Uuid,
    id_2: Uuid,
    id_1_table: Table,
    id_2_table: Table
}


