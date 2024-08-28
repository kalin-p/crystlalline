use bevy::prelude::*;
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;
use async_std::task::block_on;
use super::orgroam::sqlite_con::*;

#[derive(Resource)]
struct DBConnection {
    pool: SqlitePool
}

impl DBConnection {
    async fn new() -> Option<Self> {
        dotenv().expect(".env file not found");
        let dbpath = env::var("DATABASE_URL").expect("DATABASE_URL not set in environment");
        
        // let pool = SqlitePool::connect(dbpath.as_str()).await.unwrap();
        let pool_res = block_on( SqlitePool::connect(dbpath.as_str()));
        match pool_res {
            Ok(pool) => {
                return Some(DBConnection{pool});
            },
            Err(e) => {
                println!("error connecting to SQLite pool: {}", e);
                return None
            }
        }
    }
}

fn query_db_system (db_conn: Res<DBConnection>, kbd: Res<ButtonInput<KeyCode>>) {
    if kbd.just_pressed(KeyCode::Space) {
        let nodes = block_on(get_all_nodes(&db_conn.pool));
        for node in nodes {
            println!("{:?}", node);
        }
    }
}

pub struct DBPlugin;

impl Plugin for DBPlugin {
    fn build(&self, app: &mut App) {
        
        let conn = block_on(DBConnection::new());
        match conn {
            Some(pool) => {
                app.insert_resource(pool);
                app.add_systems(Update, query_db_system);
            },
            _ => {
                println!("could not obtain SQLite resource.");
            }
        }

    }
}

