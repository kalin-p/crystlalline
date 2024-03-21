// from this tutorial: https://www.youtube.com/watch?v=8EBsOZPGZn8


use sqlx::{sqlite::SqlitePool, Row, Column};

pub const DB_PATH: &str = "/home/kalin/.emacs.d/org-roam.db";

pub async fn open_connection(path: &str) {
    // let path = Path::new(path);
    let pool = SqlitePool::connect(path).await;
    match pool {
        Ok(pool) => {
            // pool.acquire();
            let rows = sqlx::query("select source, dest, type from links")
                .fetch_all(&pool).await.unwrap();
            for row in rows {
                // let source = row.get("source");
                // let dest = row.get("dest");
                // let link_type = row.get("type");
                let cols = row.columns();
                let vals: String = row.try_get(0).unwrap();
                // println!("source: {},\ndest: {},\ntype: {}", source, dest, link_type );
                
                println!("{:?}, {:?}", cols[0].name(), vals);
            }
            // println!("{:?}", pool)
        },
        Err(e) => println!("printing error: {}", e) 
    }

    // let query = "select name from sqlite_master where type='table'";
    // let mut res = conn.prepare(query)?;
}
    
