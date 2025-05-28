// connection_pool.rs
pub struct DatabaseConnectionPool {
    pool: Pool<ConnectionManager<dyn MyConnection>>,
}

impl DatabaseConnectionPool {
    pub fn builder() -> ConnectionPoolBuilder {
        ConnectionPoolBuilder::new()
    }
}

pub struct ConnectionPoolBuilder {
    url: String,
}

impl ConnectionPoolBuilder {
    pub fn new() -> Self {
        ConnectionPoolBuilder { url: String::new() }
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub fn build(self) -> DatabaseConnectionPool {
        let manager = ConnectionManager::<dyn MyConnection>::new(self.url);
        let pool = Pool::new(manager).unwrap();
        DatabaseConnectionPool { pool }
    }
}
