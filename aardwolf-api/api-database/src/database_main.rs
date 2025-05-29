// aardwolf-api/database-api/src/database_main.rs
//
use diesel::pg::PgConnection;
use mysql::Conn; // MySqlConnection
use diesel::sqlite::SqliteConnection;

pub enum DatabaseConnection {
    PgConnection(PgConnection),
    SqliteConnection(SqliteConnection),
    MysqlConnection(mysql::Conn), // Since diesel doesn't support MySQL
}

pub trait MyConnection {
    fn execute_query(
        &self,
        query: Box<dyn diesel::query_builder::QueryFragment<Self> + 'static>,
    ) -> Result<(), diesel::result::Error>;
    fn execute_transaction(
        &self,
        transaction: Box<dyn diesel::query_builder::QueryFragment<Self> + 'static>,
    ) -> Result<(), diesel::result::Error>;
}
