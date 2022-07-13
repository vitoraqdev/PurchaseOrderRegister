use std::error::Error;
use diesel::*;
use crate::schema::item;
use crate::schema::item::dsl::*;
use diesel::prelude::*;
use PurchaseOrderRegister::schema::item::table;

/// An abstraction for managing values in a database.
/// The objective is simplifying the process of adding, updating, and deleting values.
///
fn add_to_database<T, S>(conn: &PgConnection, table: T, value: S) -> QueryResult<()>
where
    T: Table,
    S: Insertable<T>,
{
    insert_into(table)
        .values(value)
        .get_result(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use PurchaseOrderRegister::establish_connection;
    use crate::item_model::NewItem;
    use crate::schema::item;

    #[test]
    fn add_to_item_database() {
        let conn = establish_connection();
        let item = NewItem {
            name: "Test Item".to_string(),
            price: 10.0,
            description: "Test Description".to_string(),
        };
        let result = add_to_database(&conn, item::table, item);
        assert!(result.is_ok());
    }
}