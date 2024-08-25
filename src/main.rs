use dotenvy::dotenv;
use std::env;

mod orgroam;
mod db_plugin;
use db_plugin::DBPlugin;
use orgroam::sqlite_con::*;
// use orgroam::tests::*;

use sqlx::sqlite::SqlitePool;

use bevy::prelude::*;

use config::{Config, File, FileFormat};


// #[tokio::main]
// async fn main() {
//     dotenv().expect(".env file not found");
//     let dbpath = env::var("DATABASE_URL").expect("DATABASE_URL not set in environment");
//     let pool = SqlitePool::connect(dbpath.as_str()).await.unwrap();
//     let nodes = get_all_nodes(&pool).await;
//     let links = get_all_links(&pool).await;
//     count_links_per_node(nodes, &links);

// }

fn main() {
    // let cfg_builder = Config::builder()
    //     .add_source(File::new("./config", FileFormat::Toml));
    // match cfg_builder.build() {
    //     Ok(config) => {
    //         let profile = config
    //             .get_string("database.profile")
    //             .expect("Profile not set.");
    //         let profile_setting = config
    //             .get_table(format!("database.{profile}").as_str())
    //             .expect("Profile settings not found.");
    //         let proto = &profile_setting["proto"];
    //         let path = &profile_setting["path"];
    //         let dburl = format!("{proto}://{path}");
    //     },
    //     Err(e) => {
    //         println!{"error when loading the config: {e}"};
    //     }
    // }
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DBPlugin)
        .run();
}