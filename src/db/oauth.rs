//! OAuth related database methods.

use failure::Error;

use super::models::oauth::Application;
use super::Connection;

pub fn get_application(db_con: &Connection, app_id: u64) -> Result<Option<Application>, Error> {
    unimplemented!()
}
