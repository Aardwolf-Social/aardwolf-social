// database-api/src/lib.rs
pub mod connection_pool;
pub mod database_main;
pub mod traits{
    pub mod db_handler;
}
pub mod mysql{
    pub mod mysql_main;
}
pub mod postgres{
    pub mod postgres_main;
}
pub mod sqlite{
    pub mod sqlite_main;
}
