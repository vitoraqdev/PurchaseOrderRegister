use diesel::prelude::*;
// use std::io::{stdin, Read, BufRead}
use super::schema::item;
use crate::models::item_model::*;
use text_io::read;
use PurchaseOrderRegister::establish_connection;

pub fn create_item() {
    println!("Enter the name of the item: ");
    let name: String = read!("{}\n");

    // Check if item already exists
    if is_item_in_database(&name) {
        println!("Item already exists");
    }

    println!("What is the price of {}?", name);
    let price: f64 = read!("{}\n");

    println!("What is the description of {}?", name);
    let input_description: String = read!("{}\n");

    let new_item = NewItem::new(name, price, input_description);
    add_item_to_database(&new_item);
    println!("Item {} added to database", new_item.name);
}

pub fn search_item() {
    println!("Enter the name of the item: ");
    let name: String = read!("{}\n");

    if is_item_in_database(&name) {
        let item = get_item_in_database(&name);
        println!("{:?}", item);
    } else {
        println!("Item not found");
    }
}

pub fn edit_item() {
    println!("Enter the name of the item: ");
    let name: String = read!("{}\n");

    match get_item_in_database(&name) {
        Ok(item) => {
            println!("{:?}", item);
            println!("What is the price of {}?", name);
            let price_str: String = read!("{}\n");
            let price: f64 = match price_str.parse::<f64>() {
                Ok(price) => price,
                Err(_) => {
                    println!("Invalid price");
                    return;
                }
            };

            println!("What is the description of {}?", name);
            let input_description: String = read!("{}\n");
            let input_description: String = match input_description.as_str() {
                "" => item.description,
                description => description.to_string(),
            };

            let new_item = NewItem::new(name, price, input_description);
            update_item_in_database(&new_item);
            println!("Item {} updated in database", new_item.name);
        }
        Err(_) => {
            println!("Item not found");
        }
    }
}

pub fn delete_item() {
    println!("Enter the name of the item: ");
    let name: String = read!("{}\n");

    match get_item_in_database(&name) {
        Ok(item) => {
            // ask for confirmation
            println!("{:?}", item);
            println!("Are you sure you want to delete {}?", name);
            let confirmation: String = read!("{}\n");
            if confirmation == "y" {
                delete_item_in_database(&name);
                println!("Item {} deleted from database", name);
            } else {
                println!("Aborted");
            }
        }
        Err(_) => {
            println!("Item not found");
        }
    }
}

fn add_item_to_database(new_item: &NewItem) {
    let connection = establish_connection();
    diesel::insert_into(item::table)
        .values(new_item)
        .execute(&connection)
        .expect("Error saving new item");
}

fn is_item_in_database(name: &str) -> bool {
    // TODO: DELETE THIS FUNCTION
    let connection = establish_connection();
    let item = item::table
        .filter(item::name.eq(&name))
        .first::<Item>(&connection);
    match item {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn get_item_in_database(name: &str) -> QueryResult<Item> {
    let connection = establish_connection();
    item::table
        .filter(item::name.eq(name))
        .get_result::<Item>(&connection)
}

/// Updates each field of an item in the database.
fn update_item_in_database(new_item: &NewItem) {
    let connection = establish_connection();
    diesel::update(item::table.filter(item::name.eq(&new_item.name)))
        .set((
            item::name.eq(&new_item.name),
            item::price.eq(&new_item.price),
            item::description.eq(&new_item.description),
        ))
        .execute(&connection)
        .expect("Error updating item");
}

fn delete_item_in_database(name: &str) {
    let connection = establish_connection();
    diesel::delete(item::table.filter(item::name.eq(name)))
        .execute(&connection)
        .expect("Error deleting item");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_in_database() {
        let name = "Filé de Frango Grelhado Acebolado";
        let result = is_item_in_database(name);
        assert_eq!(result, true);
    }

    #[test]
    fn test_non_existent_item_in_database() {
        let name = "Gibberish here";
        let result = is_item_in_database(name);
        assert_eq!(result, false);
    }

    #[test]
    fn test_get_item_from_database() {
        let name = "Filé de Frango Grelhado Acebolado";
        let result = get_item_in_database(name);
        assert_eq!(result.name, name);
    }

    #[test]
    fn tess() {
        let num: i32 = read!("{}\n");
        assert_eq!(num, 0.0);
    }
}
