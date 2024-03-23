use sqlx::{sqlite::SqlitePool, Row, Column, sqlite::SqliteRow};
use super::types::Link;

pub const DB_PATH: &str = "sqlite:///home/kalin/.emacs.d/org-roam.db";

pub async fn open_connection(path: &str) {
    // let path = Path::new(path);
    let pool = SqlitePool::connect(path).await;
    match pool {
        Ok(pool) => {
            // pool.acquire();
            let links: Vec<Link> = sqlx::query_as!(
                Link,
                r#"select * from links"#)
                .fetch_all(&pool).await.unwrap();
            // for row in rows {
            //     // let source = row.get("source");
            //     // let dest = row.get("dest");
            //     // let link_type = row.get("type");
            //     let cols = row.columns();
            //     for col in cols {
            //         let val = row.try_get(col.ordinal()).unwrap();
            //         println!("{:?}: {:?}",col.name(), val);
            //     }
            for l in links {
                println!("{:?}", l);
            }                
                // println!("source: {},\ndest: {},\ntype: {}", source, dest, link_type );

                // println!("{:?}", cols);
                // println!("{:?}, {:?}", cols[0].name(), vals);
            // println!("{:?}", pool)
        },
        Err(e) => println!("printing error: {}", e) 
    }

    // let query = "select name from sqlite_master where type='table'";
    // let mut res = conn.prepare(query)?;
}
    
