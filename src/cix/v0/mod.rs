mod common;
mod arb;
mod xct;
mod dpt;

use structopt::StructOpt;

#[derive(StructOpt)]
pub enum V0 {
    ARB,
    // XCT,
    // DPT,
}

pub async fn run(opt: &V0) -> ci::Result<()> {
    use V0::*;
    match opt {
        ARB => arb::run().await,
    }
}
