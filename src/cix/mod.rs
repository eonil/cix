use structopt::StructOpt;

pub mod apple;
pub mod ssh;
pub mod util;
pub mod v0;

#[derive(StructOpt)]
#[structopt(name="cix", about="CI tools collection.")]
pub enum Command {
    Apple(apple::Apple),
    SSH(ssh::SSH),
    Util(util::Util),
    V0(v0::V0),
}

pub async fn run() -> ci::Result<()> {
    let cmd = Command::from_args();
    use Command::*;
    match cmd {
        Apple(subcmd) => apple::run(&subcmd).await,
        SSH(subcmd) => ssh::run(&subcmd).await,
        Util(subcmd) => util::run(&subcmd).await,
        V0(subcmd) => v0::run(&subcmd).await,
    }
}
