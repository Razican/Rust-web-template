//! Database module.
//!
//! This module contains the basic schema for the database, as well as all the required model
//! objects to interact with it.

pub mod models;
pub mod cache;
pub mod oauth;

use std::env;

use diesel::pg::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

/// Type of database connection.
///
/// Change this to other databases such as MySQL/MariaDB.
type Connection = PgConnection;

lazy_static!{
    /// Main database connection pool.
    pub static ref CONNECTION_POOL: Pool<ConnectionManager<Connection>> = {
        let manager = ConnectionManager::new(
                env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found"));
        Pool::new(manager).expect("error creating the main database connection pool")
    };
}

/// Database schema.
#[allow(missing_docs, unused_qualifications, unused_import_braces)]
// #[recursion_limit = "1024"]
mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}
