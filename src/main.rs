mod cix;
use structopt::StructOpt;
use ci::common::CIResult;

#[derive(StructOpt)]
#[structopt(name="cix", about="CI tools collection.")]
enum Command {
    Apple(cix::apple::Apple),
    SSH(cix::ssh::SSH),
}

#[tokio::main]
async fn main() {
    run().await.unwrap();
}
async fn run() -> CIResult<()> {
    let cmd = Command::from_args();
    use Command::*;
    match cmd {
        Apple(subcmd) => cix::apple::run(&subcmd).await,
        SSH(subcmd) => cix::ssh::run(&subcmd).await,
    }
}