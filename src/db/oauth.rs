//! OAuth related database methods.


use error::*;
use super::models::oauth::Application;
use super::Connection;

pub fn get_application(db_con: &Connection, app_id: u64) -> Result<Option<Application>> {
    unimplemented!()
}
