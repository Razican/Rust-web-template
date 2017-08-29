//! OAuth cache module.

use uuid::Uuid;

use error::*;

/// Gets the hourly request count for the given application ID.
pub fn get_request_count(app_id: Uuid) -> Result<i32> {
    unimplemented!()
}

/// Adds a request to the given application ID request counter.
///
/// If the application ID does not exist, it will create a new record with 1 request and with a
/// lifetime of 1 hour.
pub fn add_request(app_id: Uuid) -> Result<()> {
    unimplemented!()
}
