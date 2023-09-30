use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, PoolError};
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn establish_pooled_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    init_pool(&database_url).expect("Failed to create pool")
}

fn init_pool(database_url: &str) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn get_from_pool(pool: &DbPool) -> Result<DbPooledConnection, PoolError> {
    let _pool = pool.get().unwrap();
    Ok(_pool)
}