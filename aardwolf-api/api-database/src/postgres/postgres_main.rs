// postgres.rs

mod schema;
use schema::posts::dsl::*;

use crate::traits::db_handler::DbHandler;
use aardwolf_api_common::models::error::Error;
use aardwolf_api_common::models::posts::Post;
pub(crate) use diesel::pg::PgConnection;
use diesel::query_builder::QueryFragment;
use diesel::query_dsl::RunQueryDsl;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

pub struct PostgresHandler {
    connection: PgConnection,
    pool: Pool<ConnectionManager<PgConnection>>,
    connection_url: String,
    is_active: bool,
    schema: Vec<String>,
}

pub trait MyConnection {
    fn execute_query(
        &mut self,
        query: Box<dyn QueryFragment<PgConnection> + 'static>,
    ) -> Result<(), DieselError>;
    fn execute_transaction(
        &mut self,
        transaction: Box<dyn QueryFragment<PgConnection> + 'static>,
    ) -> Result<(), DieselError>;
}

impl MyConnection for PostgresHandler {
    fn execute_query(
        &mut self,
        query: Box<dyn QueryFragment<PgConnection> + 'static>,
    ) -> Result<(), DieselError> {
        query.execute(&mut self.connection)?;
        Ok(())
    }

    fn execute_transaction(
        &mut self,
        transaction: Box<dyn QueryFragment<PgConnection> + 'static>,
    ) -> Result<(), DieselError> {
        transaction.execute(&mut self.connection)?;
        Ok(())
    }
}

impl DbHandler for PostgresHandler {
    fn get_posts(&self) -> Result<Vec<Box<dyn Post>>, Box<dyn Error>> {
        use schema::posts::dsl::*;

        let connection = &self.connection;

        let results = posts.load::<PgSqlPost>(connection).optional()?;

        match results {
            Some(posts) => Ok(posts.into_iter().map(Box::new).collect()),
            None => Err(Box::new(PgSqlError {
                message: "Not found".to_string(),
            })),
        }
    }

    fn create_post(&self, post: Box<dyn Post>) -> Result<(), Box<dyn Error>> {
        // implement create_post logic here
        todo!()
    }

    fn update_post(&self, post_id: i32, post: Box<dyn Post>) -> Result<(), Box<dyn Error>> {
        // implement update_post logic here
        todo!()
    }

    fn delete_post(&self, post_id: i32) -> Result<(), Box<dyn Error>> {
        // implement delete_post logic here
        todo!()
    }

    fn like_post(&self, post_id: i32) -> Result<(), Box<dyn Error>> {
        // implement like_post logic here
        todo!()
    }
}

#[derive(Debug)]
pub struct PgSqlError {
    message: String,
}

impl Error for PgSqlError {
    fn get_message(&self) -> String {
        self.message.clone()
    }

    fn get_code(&self) -> i32 {
        // implement get_code logic here
        todo!()
    }
}

#[derive(Serialize, Deserialize)]
pub struct PgSqlPost {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Post for PgSqlPost {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_content(&self) -> String {
        self.content.clone()
    }

    fn get_created_at(&self) -> String {
        self.created_at.clone()
    }

    fn get_updated_at(&self) -> String {
        self.updated_at.clone()
    }
}
