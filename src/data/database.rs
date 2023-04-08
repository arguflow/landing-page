use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use ::r2d2::PooledConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DBConection = PooledConnection<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migrations(conn: &mut DBConection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}
