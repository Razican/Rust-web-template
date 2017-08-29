// OAuth database models.

use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::super::schema::oauth_apps;

/// OAuth application.
#[derive(Debug, Queryable, Identifiable)]
#[table_name = "oauth_apps"]
pub struct Application {
    /// Application ID.
    id: Uuid,
    /// Wether the application is active or not.
    active: bool,
    /// Creation timstamp.
    creation: DateTime<Utc>,
    /// Last update timestamp.
    last_update: DateTime<Utc>,
    /// Application name.
    name: String,
    /// Application description.
    description: String,
    /// Optional URL of the application.
    url: Option<String>,
    /// Application's secret.
    api_secret: Vec<u8>,
    /// Hourly request limit.
    hourly_limit: i32,
    /// Manager ID.
    manager: i32,
}

impl Application {
    /// Gets the application ID.
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Gets wether the application is active or not.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Gets the creation timestamp.
    pub fn creation(&self) -> DateTime<Utc> {
        self.creation
    }

    /// Gets the last update timestamp.
    pub fn last_update(&self) -> DateTime<Utc> {
        self.last_update
    }

    /// Gets the application name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the application description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Gets the URL of the application.
    pub fn url(&self) -> Option<&str> {
        if let Some(ref url) = self.url {
            Some(url)
        } else {
            None
        }
    }

    /// Gets the API secret.
    pub fn api_secret(&self) -> &[u8] {
        &self.api_secret
    }

    /// Gets the hourly request limit.
    pub fn hourly_limit(&self) -> i32 {
        self.hourly_limit
    }

    // TODO get the actual manager User model.
    /// Gets the ID of the manager user.
    pub fn manager_id(&self) -> i32 {
        self.manager
    }
}

/// Structure to create a new applicaation.
#[derive(Debug, Insertable)]
#[table_name = "oauth_apps"]
pub struct NewApplication {
    /// Application name.
    name: String,
    /// Application description.
    description: String,
    /// Optional URL of the application.
    url: Option<String>,
    /// Application's secret.
    api_secret: Vec<u8>,
    /// Hourly request limit.
    hourly_limit: i32,
    /// Manager ID.
    manager: i32,
}
