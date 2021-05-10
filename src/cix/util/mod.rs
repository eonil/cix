use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Util {
    HTTPCollect(http_collect::Options),
}

pub async fn run(opt: &Util) -> ci::Result<()> {
    use Util::*;
    match opt {
        HTTPCollect(opt) => http_collect::run(opt).await,
    }
}

pub mod http_collect;
