use rocket_sync_db_pools::database;

#[database("kyward")]
pub struct DbConn(diesel::SqliteConnection);