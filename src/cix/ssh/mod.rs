use structopt::StructOpt;
use ci::common::CIResult;

#[derive(StructOpt)]
pub enum SSH {
    Jump(jump::Options),
    ReverseTunnel(tunnel::Options),
}

pub async fn run(opt: &SSH) -> CIResult<()> {
    use SSH::*;
    match opt {
        Jump(opt) => jump::run(opt).await,
        ReverseTunnel(opt) => tunnel::reverse(opt).await,
    }
}

pub mod jump;
pub mod tunnel;
