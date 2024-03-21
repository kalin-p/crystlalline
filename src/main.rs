mod orgroam;

use futures::Future;
use orgroam::sqlite_con::{ DB_PATH, open_connection };


#[tokio::main]
async fn main() {
    open_connection(DB_PATH).await;

}
