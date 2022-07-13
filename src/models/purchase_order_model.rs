use diesel::{Queryable, Insertable};


/*
INPUT TERMINAL FLOW:
input customer_name: String,
let items:: Vec<OrderItem> = vec![]
while input != "" {
    input item_name: String,
    if item_name exists in database {
        let item = get_item_from_database(item_name);
        items.push(item);
    } else {
        // TODO: finish this
        input price: f64,
        input quantity: i32,
        items.push(item);
    }

}
*/

#[derive(Queryable)]
pub struct PurchaseOrder {
    pub id: i32,
    pub customer_id: i32,
    pub date: NaiveDate,
    pub items: Vec<OrderItem>, // change to NewItem?
    pub payment_method: PaymentMethod,
    pub delivery_address: Option<Address>,
    pub motoboy_id: Option<i32>,
    pub delivery_fee: f64,
    pub discount: f64,
    pub total_price: f64,
    pub is_paid: bool,
}


#[derive(Insertable)]
#[table_name="purchase_order"]
pub struct NewPurchaseOrder<'a> {
    pub customer_id: i32,
    pub date: NaiveDate,
    pub items: Vec<OrderItem>,
    pub payment_method: PaymentMethod,
    pub delivery_address: Option<Address>,
    pub motoboy_id: Option<i32>,
    pub discount: f64,
    pub is_paid: bool,
}

impl PurchaseOrder {
    pub fn from(new_purchase_order: NewPurchaseOrder) -> Self {

        PurchaseOrder {
            id: 0,
            customer_id: new_purchase_order.customer_id,
            date: new_purchase_order.date,
            items: new_purchase_order.items,
            payment_method: new_purchase_order.payment_method,
            delivery_address: new_purchase_order.delivery_address,
            motoboy_id: new_purchase_order.motoboy_id,
            delivery_fee: 0.0,
            discount: new_purchase_order.discount,
            total_price: 0.0,
            is_paid: new_purchase_order.is_paid,
        }
    }
}