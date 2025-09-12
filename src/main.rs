 
use std::result;

use dotenv::dotenv;
use openlive::run_app;
 
 
#[tokio::main]
async fn main() {
 
    dotenv().ok();

    run_app().await.unwrap();
}