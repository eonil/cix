use std::io;
use std::net;
use std::path;
use async_ssh2_lite as assh2;
use async_io::Async;
use crate::Result;
use crate::common::CIResult;
use crate::common::MissingError;

pub struct Client {
    session: assh2::AsyncSession<net::TcpStream>,
}

pub enum Credential {
    Password(String),
    PrivateKey(String),
}

pub type TCPStream = Async<net::TcpStream>;
pub type Session = assh2::AsyncSession<net::TcpStream>;
pub type Channel = assh2::AsyncChannel<net::TcpStream>;
pub type Listener = assh2::AsyncListener<net::TcpStream>;

impl Client {
    pub async fn connect(
        addr: impl net::ToSocketAddrs, 
        username: &str,
        credential: Credential) -> Result<Client> 
    {
        let sockaddr = addr.to_socket_addrs()?.next().ok_or(Box::new(MissingError))?;
        let stream = TCPStream::connect(sockaddr).await?;
        let mut sess = Session::new(stream, None)?;
        sess.handshake().await?;
        match credential {
            Credential::Password(code) => sess.userauth_password(username, &code).await?,
            Credential::PrivateKey(code) => sess.userauth_pubkey_memory(username, None, &code, None).await?,
        };
        if !sess.authenticated() { return Err(Box::new(AuthError)) }
        Ok(Client { session: sess })
    }
    pub async fn shell(&mut self) -> CIResult<Channel> {
        let mut chan = self.session.channel_session().await?;
        let (cols,rows) = crossterm::terminal::size()?;
        let term = std::env::var("TERM")?;
        chan.request_pty(&term, None, None).await?;
        chan.request_pty_size(cols as u32, rows as u32, None, None).await?;
        chan.shell().await?;
        Ok(chan)
    }
    /// Listens on remote machine at designated address/port.
    pub async fn listen(&mut self, addr: impl net::ToSocketAddrs) -> Result<Listener> {
        let sockaddr = addr.to_socket_addrs()?.next().ok_or(Box::new(MissingError))?;
        let port = sockaddr.port();
        let host = sockaddr.ip().to_string();
        let (listener,_listen_port) = self.session.channel_forward_listen(port, Some(&host), None).await?;
        Ok(listener)
    }
}
impl Drop for Client {
    fn drop(&mut self) {
    }
}








/// Common error for `Option::None` case where a value is required.
#[derive(Debug,Clone)]
pub struct AuthError;
impl std::error::Error for AuthError {}
impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Authentication error.")
    }
}