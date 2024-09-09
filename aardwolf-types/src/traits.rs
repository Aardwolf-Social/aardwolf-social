pub trait Validate {
    type Item;
    type Error: std::error::Error;

    fn validate(self) -> Result<Self::Item, Self::Error>;
}

#[cfg(not(feature = "with-actix"))]
pub use default_impls::DbAction;

#[cfg(feature = "with-actix")]
pub use actix_web_impls::{DbAction, DbActionError};

#[cfg(not(feature = "with-actix"))]
mod default_impls {
    use diesel::pg::PgConnection;

    pub trait DbAction {
        type Item;
        type Error: std::error::Error;

        fn db_action(self, conn: &PgConnection) -> Result<Self::Item, Self::Error>;
    }
}

#[cfg(feature = "with-actix")]
mod actix_web_impls {
    use actix_web::{error::BlockingError, web::block};
    use diesel::r2d2::ConnectionManager;
    use diesel::PgConnection;
    use futures::future::{BoxFuture, FutureExt, TryFutureExt};
    use r2d2::Pool;
    use std::error::Error as StdError;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum DbActionError<E>
    where
        E: StdError,
    {
        #[error("Error in Db Action, {}", _0)]
        Error(#[source] E),

        #[error("Error in pooling, {}", _0)]
        Pool(#[source] r2d2::Error),

        #[error("Db Action was canceled")]
        Canceled,
    }

    impl<E> From<BlockingError> for DbActionError<E>
    where
        E: StdError,
    {
        fn from(_: BlockingError) -> Self {
            DbActionError::Canceled
        }
    }

    impl<E> From<r2d2::Error> for DbActionError<E>
    where
        E: StdError,
    {
        fn from(e: r2d2::Error) -> Self {
            DbActionError::Pool(e)
        }
    }

    pub trait DbAction {
        type Item: Send + 'static;
        type Error: StdError + Send;

        fn db_action(self, conn: &mut PgConnection) -> Result<Self::Item, Self::Error>;

        fn run(
            self,
            pool: Pool<ConnectionManager<PgConnection>>,
        ) -> BoxFuture<'static, Result<Self::Item, DbActionError<Self::Error>>>
        where
            Self: Sized + Send + 'static,
        {
            block(move || -> Result<Self::Item, DbActionError<Self::Error>> {
                let conn = &mut *pool.get()?;

                self.db_action(conn).map_err(DbActionError::Error)
            })
            .map_err(DbActionError::from)
            .map(|result| result.and_then(|inner| inner))
            .boxed()
        }
    }
}

