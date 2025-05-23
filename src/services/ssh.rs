use std::net::TcpStream;
use std::path::Path;
use ssh2::{Session, Channel};
use crate::config::SSHConfig;
use crate::error::AppError;

pub struct Ssh {}

impl Ssh {
    pub fn open_channel(cfg: &SSHConfig) -> Result<Channel, AppError> {
        let tcp = TcpStream::connect(
            format!("{}:{}", cfg.ssh_server, cfg.ssh_port))
            .unwrap();

        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();

        // so ~ doesn't work in Path but it does in File::read....
        //let pk_path = home_dir().unwrap().join(".ssh/id_ed25519");
        // TODO MAKE A BETTER SOLUTION FOR THIS THAT HANDLES ~ FROM EXTERNAL
        let pk_path = Path::new(&cfg.key_path);

        // Auth
        sess.userauth_pubkey_file(
            &cfg.ssh_username,
            None,
            &pk_path,
            None)
            .unwrap();

        sess.channel_session().map_err(|_| AppError::SessionChannel)
    }
}
