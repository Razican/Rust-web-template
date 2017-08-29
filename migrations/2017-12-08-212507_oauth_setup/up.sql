-- Add the UUID-OSSP extension.
CREATE EXTENSION "uuid-ossp";

-- Create the users table.
--
-- It might make sense not to remove users and just deactivate them, but that will
-- cause problems with privacy. There must be a way to easily remove all user data
-- if requested by the user.
CREATE TABLE users (
    id SERIAL NOT NULL PRIMARY KEY,
    active BOOLEAN DEFAULT FALSE, -- Should be activated by email.
    creation TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
    last_active TIMESTAMP(3) WITH TIME ZONE DEFAULT NULL,
    email TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL UNIQUE,
    password BYTEA NOT NULL
    -- More fields can be added here.
);

-- Create the oauth applications table.
CREATE TABLE oauth_apps (
    id UUID NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    active BOOLEAN DEFAULT FALSE, -- Should be activated by email.
    creation TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
    last_update TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    url TEXT, -- Optional
    api_secret BYTEA NOT NULL,
    hourly_limit INTEGER NOT NULL CHECK(hourly_limit > 0), -- Hourly request limit
    manager INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE -- All apps should have a manager
);

-- Create the OAuth application deactivation function for the trigger.
CREATE FUNCTION deactivate_apps() RETURNS TRIGGER AS $func$
BEGIN
    UPDATE oauth_apps SET active = FALSE WHERE manager = NEW.id; -- deactivate all applications for current manager
    RETURN NEW;
END;
$func$ LANGUAGE plpgsql;

-- Create trigger so that if a manager is deactivated, all its applications get deactivated too.
CREATE TRIGGER deactivate_inactive_manager_apps
    AFTER UPDATE OF active ON users -- Once the 'active' field update is successful
    FOR EACH ROW -- For each user in a bulk user deactivation
    WHEN (OLD.active = TRUE AND NEW.active = FALSE) -- When the user gets deactivated
    EXECUTE PROCEDURE deactivate_apps();
