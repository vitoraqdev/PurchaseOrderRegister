#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod item;
pub mod models;
pub mod schema;
mod database_manager;

use self::models::*;

// mod item;
//
// use models::address_model::*;
use text_io::*;

fn main() {
    // 1. Orders
    // 2. Customers
    // 3. Items
    // 4. Addresses
    // 5. Motoboys

    loop {
        println!("\n\n");
        println!("1. Orders");
        println!("2. Customers");
        println!("3. Items");
        println!("4. Addresses");
        println!("5. Motoboys");
        println!("6. Exit");
        println!("\n");
        println!("Enter your choice: ");
        let choice = read!("{}\n");
        println!("\n");
        match choice {
            1 => {
                todo!()
            }
            2 => {
                todo!()
            }
            3 => {
                println!("1. Create Item");
                println!("2. Search Item");
                println!("3. Edit Item");
                println!("4. Delete Item");
                println!("5. Back");
                println!("\n");
                println!("Enter your choice: ");
                let choice: i32 = read!("{}\n");
                match choice {
                    1 => {
                        item::create_item();
                    }
                    2 => {
                        item::search_item();
                    }
                    3 => {
                        item::edit_item();
                    }
                    4 => {
                        item::delete_item();
                    }
                    5 => {
                        continue;
                    }
                    _ => {
                        println!("Not implemented")
                    }
                }
            }
            4 => {
                todo!()
            }
            5 => {
                todo!()
            }
            6 => {
                break;
            }
            _ => {
                continue;
            }
        }
    }
}
