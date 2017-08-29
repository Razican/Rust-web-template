-- Remove the OAuth application deactivation trigger.
DROP TRIGGER deactivate_inactive_manager_apps ON users;

-- Remove application deactivation function.
DROP FUNCTION deactivate_apps();

-- Remove the oauth applications table.
DROP TABLE oauth_apps;

-- Remove the users table.
DROP TABLE users;

-- Remove the UUID-OSSP extension.
DROP EXTENSION "uuid-ossp";
