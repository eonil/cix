const HELP: &'static str = r#"

Opens a reverse-proxy tunnel.

EXAMPLE 1

    cix ssh reverse-tunnel user1 host2:22 0.0.0.0:19998 localhost:19998

Please note that incoming connections only from `127.0.0.1` on remote machine 
will be accepted by default. You need to modify `sshd`'s `GatewayPorts`
configuration to open connections from other remote machines.
https://superuser.com/questions/588591/how-to-make-an-ssh-tunnel-publicly-accessible

"#;

use structopt::StructOpt;
use ci::common::CIResult;

/// Run a reverse-tunnel.
/// You need to provide path to private-key for remote machine authentication.
#[derive(StructOpt)]
pub struct Options {
    username: String,
    gateway_addr: String,
    /// Listening address/port on remote machine.
    remote_in_addr: String,
    /// Connecting address/port on local machine.
    local_out_addr: String,
}

pub async fn reverse(opt: &Options) -> CIResult<()> {
    use std::fs;
    use std::path;
    use std::net;
    use std::io::{Read};
    use ci::ssh2;

    // let password = std::env::var("CIX_SSH_PASSWORD")?;
    let private_key_path = std::env::var("CIX_SSH_PRIVATE_KEY_PATH")?;

    // pub fn connect(addr: impl net::ToSocketAddrs, username: &str, password: &str) -> CIResult<Client> {
    let key_path = path::PathBuf::from(private_key_path);
    let mut key_code = String::new();
    fs::File::open(key_path)?.read_to_string(&mut key_code)?;
    let cred = ssh2::Credential::PrivateKey(key_code);
    let mut client = ssh2::Client::connect(&opt.gateway_addr, &opt.username, cred).await?;
    let mut listener = client.listen(&opt.remote_in_addr).await?;
    let keepalive = true;
    
    use ci::ssh2::TCPStream;
    use net::ToSocketAddrs;
    while keepalive {
        println!("listening...");
        let mut remote_chan: ssh2::Channel = listener.accept().await?;
        println!("accepted.");
        let local_addr = opt.local_out_addr.to_socket_addrs()?.next().ok_or(Box::new(ci::common::MissingError))?;
        let mut local_chan: TCPStream = TCPStream::connect(local_addr).await?;
        println!("start of a tunnel.");
        match run_tunnel(&mut remote_chan, &mut local_chan).await {
            Err(x) => eprintln!("error in a tunnel: {}", x),
            Ok(_) => {},
        };
        println!("end of a tunnel.");
    };
    Ok(())
}

async fn run_tunnel(remote_chan: &mut ci::ssh2::Channel, local_chan: &mut ci::ssh2::TCPStream) -> CIResult<()> {
    use futures_lite::io::{AsyncReadExt, AsyncWriteExt};
    use futures::future::FutureExt;
    let mut remote_buffer = [0u8; 4096];
    let mut local_buffer = [0u8; 4096];
    'tunnel: loop {
        let mut read_remote = remote_chan.read(&mut remote_buffer).fuse();
        let mut read_local = local_chan.read(&mut local_buffer).fuse();
        futures::select! {
            result = read_remote => match result {
                Err(x) => {
                    eprintln!("error in a tunnel: {}", x);
                    break 'tunnel;
                },
                Ok(n) => {
                    if remote_chan.eof() { break 'tunnel }
                    let subbuf = &remote_buffer[0..n];
                    local_chan.write_all(subbuf).await?;
                },
            },
            result = read_local => match result {
                Err(x) => {
                    eprintln!("error in a tunnel: {}", x);
                    break 'tunnel;
                },
                Ok(n) => {
                    let subbuf = &local_buffer[0..n];
                    remote_chan.write_all(subbuf).await?;
                },
            },
        };
    };
    Ok(())
}