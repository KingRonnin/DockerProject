use diesel::sqlite::SqliteConnection;
use diesel::Connection;

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("db.sqlite3").expect("Failed to connect database")
}