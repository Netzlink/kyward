use rocket_sync_db_pools::database;

#[database("dev")]
pub struct DbConn(diesel::SqliteConnection);