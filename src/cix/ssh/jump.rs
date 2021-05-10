const HELP: &'static str = r#"

EXAMPLE 1

    export CIX_SSH_PRIVATE_KEY_PATH=./your/private/key/file
    cix ssh jump user1 3.3.3.3:22 4.4.4.4:22

"#;

use structopt::StructOpt;
use ci::common::CIResult;

/// Run a jump-SSH (multi-hop) connection.
#[derive(StructOpt)]
#[structopt(help = HELP)]
pub struct Options {
    username: String,
    gateway_addr: String,
    dest_addr: String,
}

pub async fn run(opt: &Options) -> CIResult<()> {
    use std::fs;
    use std::path;
    use std::io::{Read};
    use ci::ssh2;
    use futures_lite::io::{AsyncReadExt, AsyncWriteExt};

    // let password = std::env::var("CIX_SSH_PASSWORD")?;
    let private_key_path = match std::env::var("CIX_SSH_PRIVATE_KEY_PATH") {
        Err(e) => return Err(Box::new(MissingSSHPrivateKeyPath)),
        Ok(x) => x,
    };
    // pub fn connect(addr: impl net::ToSocketAddrs, username: &str, password: &str) -> CIResult<Client> {
    let key_path = path::PathBuf::from(private_key_path);
    let mut key_code = String::new();
    fs::File::open(key_path)?.read_to_string(&mut key_code)?;
    let cred = ssh2::Credential::PrivateKey(key_code);
    let mut client = ssh2::Client::connect(&opt.gateway_addr, &opt.username, cred).await?;
    let mut chan: ssh2::Channel = client.shell().await?;
    chan.write_all(b"ls -la\nexit\n").await?;
    let mut output = Vec::<u8>::new();
    chan.read_to_end(&mut output).await?;
    println!("{}", String::from_utf8(output)?);
    Ok(())
}

use std::fmt;
#[derive(Debug)]
struct MissingSSHPrivateKeyPath;
impl std::error::Error for MissingSSHPrivateKeyPath {}
impl fmt::Display for MissingSSHPrivateKeyPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("`CIX_SSH_PRIVATE_KEY_PATH` environment variable is missing.")
    }
}

