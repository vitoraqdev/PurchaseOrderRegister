use diesel::{Queryable, Insertable};


#[derive(Queryable)]
pub struct Neighborhood {
    pub id: i32,
    pub neighborhood: String,
    pub delivery_fee: f64,
}


#[derive(Insertable)]
#[table_name="neighborhood"]
pub struct NewNeighborhood {
    pub neighborhood: str,
    pub delivery_fee: f64,
}
