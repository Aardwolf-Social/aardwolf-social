use diesel::mysql::MysqlConnection;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use aardwolf_models::schema;

pub enum DatabaseConnectionManager {
    PgConnectionManager(ConnectionManager<PgConnection>),
    SqliteConnectionManager(ConnectionManager<SqliteConnection>),
    MysqlConnectionManager(ConnectionManager<MysqlConnection>),
}

pub fn establish_connection(url: &str) -> Result<DatabaseConnectionManager, diesel::result::Error> {
    match url {
        url if url.starts_with("postgres://") => {
            let manager = ConnectionManager::<PgConnection>::new(url);
            Ok(DatabaseConnectionManager::PgConnectionManager(manager))
        }
        url if url.starts_with("sqlite://") => {
            let manager = ConnectionManager::<SqliteConnection>::new(url);
            Ok(DatabaseConnectionManager::SqliteConnectionManager(manager))
        }
        url if url.starts_with("mysql://") => {
            let manager = ConnectionManager::<MysqlConnection>::new(url);
            Ok(DatabaseConnectionManager::MysqlConnectionManager(manager))
        }
        _ => Err(diesel::result::Error::QueryBuilderError(
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Invalid connection URL")),
        )),
    }
}
