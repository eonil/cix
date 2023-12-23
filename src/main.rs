#![allow(dead_code)]

mod cix;

#[tokio::main]
async fn main() {
    match cix::run().await {
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        },
        Ok(_) => {
            std::process::exit(0);
        },
    }
}