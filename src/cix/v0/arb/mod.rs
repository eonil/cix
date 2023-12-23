use async_std as astd;
use astd::task;
use astd::io::ReadExt;
use super::xct;
mod ui;

type TCPListener = astd::net::TcpListener;
type TCPStream = astd::net::TcpStream;

pub async fn run() -> ci::Result<()> {
    eprintln!("CIX V0 ARB");
    let listen = TCPListener::bind("0.0.0.0:19990").await?;
    loop {
        let (mut chan, _addr) = listen.accept().await?;
        task::spawn(async move {
            match run_xct(&mut chan).await {
                Err(x) => eprintln!("{}", x),
                Ok(_) => {},
            };
        });
    }
}

async fn run_xct(chan: &mut TCPStream) -> ci::Result<()> {
    const MSG_SIZE_MAX: usize = 128usize * 1024usize;
    let mut buf = [0u8; MSG_SIZE_MAX];
    loop {
        let mut lenbuf = [0u8; 4];
        chan.read_exact(&mut lenbuf).await?;
        let msglen = u32::from_be_bytes(lenbuf) as usize;
        if msglen > MSG_SIZE_MAX { return Err(Box::new(TooBigMessage)) }
        let mut msgbuf = &mut buf[0..msglen];
        chan.read_exact(&mut msgbuf).await?;
        let report = serde_json::from_slice::<xct::Report>(msgbuf)?;
        use xct::Report::*;
        match report {
            Reply(id,ok) => {},
            ExecutionLaunch => {},
            ExecutionProgress(kind, chunk) => {}
            ExecutionExit(code) => {},
            PostInit => {},
            PostProgress(progress) => {},
            PostComplete(result) => return Ok(()),
        };
    }
}

enum Role {
    Admin,
    Developer,
    Tester,
}

struct User {
    name: String,
    role: Role,
}



use std::fmt;
#[derive(Debug)]
struct TooBigMessage;
impl std::error::Error for TooBigMessage {}
impl fmt::Display for TooBigMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Too big message. Cannot be decoded. CIX limits single message size <= 128KiB.")
    }
}