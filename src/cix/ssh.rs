use structopt::StructOpt;
use ci::common::CIResult;

#[derive(StructOpt)]
pub enum SSH {
    Jump(SSHJumpOption)
}

#[derive(StructOpt)]
pub struct SSHJumpOption {
    gateway_host: String,
    gateway_port: u16,
    dest_host: String,
    dest_port: u16,    
}

pub async fn run(opt: &SSH) -> CIResult<()> {
    use SSH::*;
    match opt {
        Jump(opt) => jump(opt).await,
    }
}



// use std::net::TcpStream;
// use std::net::ToSocketAddrs;

use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::time::Duration;
// use async_ssh2_lite;
// use async_ssh2_lite::*;



async fn jump(opt: &SSHJumpOption) -> CIResult<()> {
    let gateway_addr = [&opt.gateway_host, opt.gateway_port.to_string().as_str()].join(":");
    let dest_addr = [&opt.dest_host, opt.dest_port.to_string().as_str()].join(":");
    let mut chan = TcpStream::connect(gateway_addr).await?;
    chan.write_all(b"A").await?;

    tokio::time::sleep(Duration::from_millis(1000 * 10)).await;
    Ok(())
}