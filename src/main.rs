use anyhow::{Context, Result};
use kdam::TqdmParallelIterator;
use rayon::prelude::*;
use ssh2::Session;
use std::io::Read;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

const BASE_IP: &str = "139.179.121.";
const PORT: u16 = 22;
const TIMEOUT: u64 = 5;
const USERNAME: &str = "username";
const PASSWORD: &str = "password";
const COMMAND: &str = "nvidia-smi -L";

fn get_gpu_info(host: &str) -> Result<String> {
    // Resolve the hostname and port into a SocketAddr
    let addr = host
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| anyhow::anyhow!("Unable to resolve hostname: {}", host))?;
    let timeout = Duration::new(TIMEOUT, 0);
    let tcp = TcpStream::connect_timeout(&addr, timeout)
        .with_context(|| format!("Failed to connect to {}", host))?;

    // Create a new SSH session
    let mut sess = Session::new().context("Failed to create SSH session")?;
    sess.set_tcp_stream(tcp);

    // Attempt to authenticate with the first available method
    sess.handshake().context("SSH handshake failed")?;
    sess.userauth_password(USERNAME, PASSWORD)
        .with_context(|| format!("Authentication failed for user {}", USERNAME))?;

    // Execute a command
    let mut channel = sess
        .channel_session()
        .context("Failed to create SSH channel")?;
    channel
        .exec(COMMAND)
        .with_context(|| format!("Failed to execute command: {}", COMMAND))?;

    // Read the command output into the String
    let mut output = String::new();
    channel
        .read_to_string(&mut output)
        .with_context(|| "Failed to read command output")?;
    channel.wait_close().context("Failed to close channel")?;
    Ok(output)
}

fn main() -> Result<()> {
    let machines = (40..80).collect::<Vec<_>>();

    let infos = machines
        .par_iter()
        .tqdm()
        .filter_map(|i| {
            let host = format!("{}{}:{}", BASE_IP, i, PORT);
            let info_result = match get_gpu_info(&host) {
                Ok(info) => Some((host, info)),
                Err(_) => None,
            };
            info_result
        })
        .collect::<Vec<_>>();

    // Print the GPU info
    for (host, info) in infos {
        println!("{}: {}", host, info.trim());
    }
    Ok(())
}
