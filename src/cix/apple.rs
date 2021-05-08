use structopt::StructOpt;
use ci::common::CIResult;

#[derive(StructOpt)]
pub enum Apple {
    MissingDSYMCheck {
        ipa: String,
        dsym: String,
    }
}

pub async fn run(opt: &Apple) -> CIResult<()> {
    use Apple::*;
    match opt {
        MissingDSYMCheck { ipa, dsym } => missing_dsym_check(&ipa, &dsym).await?,
    }
    Ok(())
}

async fn missing_dsym_check(ipa: &str, dsym: &str) -> CIResult<()> {
    // let uuids = ci::apple::scan_uuids(ipa);
    Ok(())
}