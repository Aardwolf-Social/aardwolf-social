<<<<<<< HEAD

// API database modules
mod api_db_routes;
=======
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
>>>>>>> d1b24ddd5f52965698d08e47af9333a61cbcc526
