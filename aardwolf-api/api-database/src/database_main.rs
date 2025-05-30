use config::{Config, File};
use diesel::r2d2::{ConnectionManager, Error, Pool};
use diesel::{prelude::*, r2d2};

/// Configure database from user config
fn configure_database() -> Result<(String, String, i32, String, String, String), config::ConfigError>
{
    let config = Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()?;

    let database_type: String = config.get("Database.type").unwrap().into();
    let host: String = config.get("Database.host").unwrap().into();
    let port: i32 = config.get("Database.port").unwrap().into();
    let username: String = config.get("Database.username").unwrap().into();
    let password: String = config.get("Database.password").unwrap().into();
    let database: String = config.get("Database.database").unwrap().into();

    Ok((database_type, host, port, username, password, database))
}

/// Trait for database operations
pub trait DbConnection: Sized {
    /// Execute a query on the database
    fn execute(&self, query: &str) -> Result<(), diesel::result::Error>;

    /// Begin a test transaction
    fn begin_test_transaction(&self) -> Result<(), diesel::result::Error>;

    /// Commit a test transaction
    fn test_transaction(&self) -> Result<(), diesel::result::Error>;
}

/// Connection pool for managing database connections
pub struct ConnectionPool<DB: diesel::Connection + r2d2::R2D2Connection + 'static> {
    pool: Pool<ConnectionManager<DB>>,
}

impl<DB: diesel::Connection + r2d2::R2D2Connection + 'static> ConnectionPool<DB> {
    /// Create a new connection pool
    pub fn new(database_url: &str) -> Result<Self, Error> {
        let manager = ConnectionManager::<DB>::new(database_url);
        let pool = Pool::builder()
            .connection_customizer(Box::new(|conn: DB| {
                conn.execute("SET TIME ZONE 'UTC';").unwrap();
                conn
            }))
            .build(manager)?;

        Ok(ConnectionPool { pool })
    }

    /// Get a connection from the pool
    pub fn get(&self) -> Result<DB, Error> {
        self.pool.get()
    }
}

impl<DB: diesel::Connection + r2d2::R2D2Connection + 'static> DbConnection for ConnectionPool<DB> {
    fn execute(&self, query: &str) -> Result<(), diesel::result::Error> {
        let conn = self.get()?;
        diesel::sql_query(query).execute(&*conn)?;
        Ok(())
    }

    fn begin_test_transaction(&self) -> Result<(), diesel::result::Error> {
        let conn = self.get()?;
        conn.begin_test_transaction()
    }

    fn test_transaction(&self) -> Result<(), diesel::result::Error> {
        let conn = self.get()?;
        conn.test_transaction()
    }
}
