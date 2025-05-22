use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::net::TcpStream;
use std::path::Path;
use clap::Args;
use ssh2::Session;
use dirs::home_dir;
use crate::error::AppError;
use crate::config::{PartialConfig, SSHConfig};

use super::CommandRunner;

#[derive(Args, Debug)]
pub struct NewCommand {
    pub repo_name: String,

    #[arg(short = 'k', long = "key-path")]
    pub key_path: Option<String>,

    #[arg(short = 'u', long = "user")]
    pub ssh_username: Option<String>,

    #[arg(short = 's', long = "server")]
    pub ssh_server: Option<String>,

    #[arg(short = 'p', long = "port")]
    pub ssh_port: Option<String>,

    #[arg(short = 'r', long = "remote-path")]
    pub remote_repo_path: Option<String>,
}

impl NewCommand {
    pub fn build_ssh_config(&self) -> Result<SSHConfig, AppError> {
        let path = home_dir().unwrap().join(".config/mygit/config.yaml");
        let file = File::open(path).unwrap(); // dangerous
        let reader = BufReader::new(file); 
        let partial: PartialConfig = serde_yaml::from_reader(reader).unwrap();

        Ok(
            SSHConfig {
                key_path:
                    self.key_path.clone()
                    .or(partial.key_path)
                    .ok_or(AppError::MissingArgument("key_path".into()))?,
                ssh_username:
                    self.ssh_username.clone()
                    .or(partial.ssh_username)
                    .ok_or(AppError::MissingArgument("ssh_username".into()))?,
                ssh_server:
                    self.ssh_server.clone()
                    .or(partial.ssh_server)
                    .ok_or(AppError::MissingArgument("ssh_server".into()))?,
                ssh_port:
                    self.ssh_port.clone()
                    .or(partial.ssh_port)
                    .ok_or(AppError::MissingArgument("ssh_port".into()))?,
                remote_repo_path:
                    self.remote_repo_path.clone()
                    .or(partial.remote_repo_path)
                    .ok_or(AppError::MissingArgument("remote_repo_path".into()))?,
            }
        )
    }
}

// Maybe we can embed requires ssh into the command trait via function

impl CommandRunner for NewCommand {
    fn execute(&self) -> Result<(), AppError> {
        let cfg = self.build_ssh_config()?;

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

        let mut channel = sess.channel_session().unwrap();
        let command = format!("cd /usr/local/git && git init --bare {}.git", self.repo_name);

        // We should unwind our actions up to this point if it fails
        match channel.exec(&command) {
            Ok(()) => {},
            Err(err) => {
                dbg!(&err);
                panic!("Failed to create remote");
            },
        };

        let mut output = String::new();
        channel.read_to_string(&mut output).unwrap();
        println!("Output: {}", output);


        channel.wait_close().unwrap();
        println!("Exit Status: {}", channel.exit_status().unwrap());
        Ok(())
    }
}
