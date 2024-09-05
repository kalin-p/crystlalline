use bevy::prelude::*;
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;
use async_std::task::block_on;
use super::orgroam::sqlite_con::*;
use sexp::*;
use sexp::Atom;

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
        // let nodes = block_on(get_all_nodes(&db_conn.pool));
        // for node in nodes {
        //     println!("{:?}", node);
        // }

        let n = block_on(get_one_node(&db_conn.pool));
        match n.properties {
            Some(props) => {
                let parsed_props = parse(props.as_str()).ok().unwrap();
                match parsed_props {
                    Sexp::List(data1) => {
                        for d in data1 {
                            match d {
                                Sexp::List(cons) => {
                                    let conscopy = cons.clone();
                                    match &cons[0] {
                                        Sexp::Atom(p) => {
                                            match &p {
                                                Atom::S(pname) => {
                                                    if *pname == "ALLTAGS".to_string() {
                                                        for a in &cons {
                                                            println!("{:?}", a);
                                                        }
                                                    }
                                                },
                                                _ => {}
                                            }
                                        },
                                        _ => {}
                                    }
                                },
                                _ => {}
                            }
                        }
                    },
                    _ => {}
                }
            },
            None => {
                println!("Node properties not found.");
            }
        }

        // let specific_id = String::from("'\"37e91eab-7baa-42e2-995d-d3cd606c3526\"'");
        // println!("{}", specific_id);
        // println!(r#"select * from nodes where title = "2024-03-28"' "#)
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

