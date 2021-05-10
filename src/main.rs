#![allow(dead_code)]

mod cix;
use structopt::StructOpt;
use ci::common::CIResult;

#[derive(StructOpt)]
#[structopt(name="cix", about="CI tools collection.")]
enum Command {
    Apple(cix::apple::Apple),
    SSH(cix::ssh::SSH),
    Util(cix::util::Util),
}

#[tokio::main]
async fn main() {
    match run().await {
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        },
        Ok(_) => {
            std::process::exit(0);
        },
    }
}
async fn run() -> CIResult<()> {
    let cmd = Command::from_args();
    use Command::*;
    match cmd {
        Apple(subcmd) => cix::apple::run(&subcmd).await,
        SSH(subcmd) => cix::ssh::run(&subcmd).await,
        Util(subcmd) => cix::util::run(&subcmd).await,
    }
}
