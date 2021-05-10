const HELP: &'static str = r#"

Opens a reverse-proxy tunnel.

EXAMPLE 1

    cix ssh reverse-tunnel user1 host2:22 0.0.0.0:19998 localhost:19998

Please note that incoming connections only from `127.0.0.1` on remote machine 
will be accepted by default. You need to modify `sshd`'s `GatewayPorts`
configuration to open connections from other remote machines.
https://superuser.com/questions/588591/how-to-make-an-ssh-tunnel-publicly-accessible

"#;

/// Run a reverse-tunnel.
/// You need to provide path to private-key for remote machine authentication.
#[derive(StructOpt)]
pub struct Options {
    collect_port: u16,
    replay_port: u16,
}

pub async fn run(opt: &Options) -> ci::Result<()> {
    http_collect(opt.collect_port, opt.replay_port).await
}



use std::net;
use std::sync;
use async_io::Async;
use structopt::StructOpt;
use futures_lite::io::{AsyncReadExt, AsyncWriteExt};

type TCPStream = Async<net::TcpStream>;
type TCPListener = Async<net::TcpListener>;
type Message = Vec<u8>;
type MessageVec = Vec<Message>;
type MutexMessageVec = sync::Mutex<MessageVec>;

/// Collect all incoming HTTP messages and replay them.
/// This always reply 200 OK.
async fn http_collect(collect_port: u16, replay_port: u16) -> ci::Result<()> {
    let msgx = sync::Mutex::new(MessageVec::new());
    futures::join!(collect(collect_port, &msgx), replay(replay_port, &msgx));
    Ok(())
}

async fn collect(port:u16, msgx: &MutexMessageVec) -> ci::Result<()> {
    let listen = TCPListener::bind(([0,0,0,0],port))?;
    loop {
        let (mut chan, addr): (TCPStream, net::SocketAddr) = listen.accept().await?;
        chan.write_all(b"HTTP 200 OK\r\nContent-Length: 0\r\n\r\n").await?;
        chan.flush().await?;
        let mut msg = Vec::<u8>::new();
        chan.read_to_end(&mut msg).await?;
        chan.close().await?;
        msgx.lock().unwrap().push(msg);
        println!("collected from {}", addr);
    }
}

async fn replay(port:u16, msgx: &MutexMessageVec) -> ci::Result<()> {
    let listen = TCPListener::bind(([0,0,0,0],port))?;
    loop {
        let (mut chan, addr): (TCPStream, net::SocketAddr) = listen.accept().await?;
        let msgs = msgx.lock().unwrap();
        let len = msgs.iter().map(|msg| msg.len()).sum::<usize>();
        let head = format!("HTTP 200 OK\r\nContent-Length: {}\r\n\r\n", len);
        chan.write_all(head.as_bytes()).await?;
        for msg in msgs.iter() {
            chan.write_all(msg).await?;
        }
        chan.flush().await?;
        chan.close().await?;
        println!("scattered to {}", addr);
    }
}