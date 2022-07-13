use diesel::{Queryable, Insertable};
use crate::models::neighborhood_model::NewNeighborhood;

/*



*/



#[derive(Queryable)]
pub struct Address {
    pub id: i32,
    pub street: String,
    pub number: String,
    pub neighborhood0: Neighborhood,
    pub complement: String,
    pub observation: String,
    pub delivery_fee: f64,
}


#[derive(Insertable)]
#[table_name="address"]
/// Struct to insert a new address into the database.
///
/// The neighborhood is input as a string, and is converted to a neighborhood.
///
/// The delivery fee is automatically set by the Neighborhood model.
///
/// Example:
/// NewAddress::new("Main St", "123", "Alpha", "Apt. 101", "Near the mall")
pub struct NewAddress {
    pub street: String,
    pub number: String,
    pub neighborhood: NewNeighborhood,
    pub complement: String,
    pub observation: String,
    pub delivery_fee: f64,
}

impl NewAddress {
    pub fn new(street: String, number: String, neighborhood: NewNeighborhood, complement: String, observation: String) -> Self {
        NewAddress {
            street,
            number,
            neighborhood,
            complement,
            observation,
            delivery_fee: neighborhood.delivery_fee,
        }
    }
}
