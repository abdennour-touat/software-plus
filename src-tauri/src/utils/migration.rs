pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::db::ConnPool;

pub fn run_migration(conn: &ConnPool) {
    // Get a connection from the connection pool
    conn.get()
        .map_err(|e| panic!("Failed to get a connection from the connection pool. {}", e))
        .unwrap()
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| panic!("Failed to migrations. {}", e))
        .unwrap();
}
