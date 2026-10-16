use uuid::{uuid, Uuid};
use chrono::{DateTime, Utc};


enum Table {
    Person,
    Place,
    Note, 
}


pub struct Person {
    id: Uuid,
    name: String,
    last_name: String,
    phone_number: String,
    email_address: String,
    description: String
}

pub struct Place {
    id: Uuid,
    display_name: String,
    geo_coords: String,
    description: String,
}

pub struct Note {
    id: Uuid,
    title: String,
    content: String,
    edited: DateTime<Utc>
}

pub struct Relation {
    id: Uuid,
    id_1: Uuid,
    id_2: Uuid,
    id_1_table: Table,
    id_2_table: Table
}