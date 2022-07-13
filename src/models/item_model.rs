extern crate diesel;
use super::super::schema::item;
use diesel::prelude::*;

#[derive(Debug, Queryable)]
pub struct Item {
    id: i32,
    pub name: String,
    pub price: f64,
    pub description: String,
}

#[derive(Debug, Insertable)]
#[table_name = "item"]
pub struct NewItem {
    pub name: String,
    pub price: f64,
    pub description: String,
}

impl NewItem {
    pub fn new(name: String, price: f64, description: String) -> Self {
        NewItem {
            name,
            price,
            description,
        }
    }

    pub fn from(item: Item) -> Self {
        NewItem {
            name: item.name,
            price: item.price,
            description: item.description,
        }
    }
}
