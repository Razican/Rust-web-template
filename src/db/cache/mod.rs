//! Database cache module

pub mod oauth;

use std::env;

use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;

lazy_static!{
    /// Redis cache connection pool.
    pub static ref CONNECTION_POOL: Pool<RedisConnectionManager> = {
        let manager = RedisConnectionManager::new(
                env::var("REDIS_DATABASE")
                    .expect("REDIS_DATABASE environment variable not found").as_str())
            .expect("error creating the Redis connection manager");
        Pool::new(manager).expect("error creating the Redis connection pool")
    };
}
