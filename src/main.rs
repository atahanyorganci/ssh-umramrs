use anyhow::{Context, Result};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
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
    let machines = (0..256).collect::<Vec<_>>();
    let mpb = MultiProgress::new();
    let style = ProgressStyle::default_bar()
        .template("{prefix:.bold.dim} {spinner:.green} {msg}").unwrap();

    let main_pb = mpb.add(ProgressBar::new(machines.len() as u64));
    let main_style = ProgressStyle::default_bar()
        .template("{prefix:.bold} {bar:40.cyan/blue} {pos:>7}/{len:7}").unwrap();
    main_pb.set_style(main_style);
    main_pb.set_prefix("SSH Progress: ");


    let infos = machines
        .par_iter()
        .filter_map(|i| {
            let pb = mpb.add(ProgressBar::new(1));
            pb.set_style(style.clone());
            let host = format!("{}{}:{}", BASE_IP, i, PORT);
            pb.set_message(format!("{host}: connecting..."));
            let info_result = match get_gpu_info(&host) {
                Ok(info) => Some((host, info)),
                Err(_) => None,
            };
            pb.inc(1);
            pb.finish_and_clear();
            main_pb.inc(1);
            info_result
        })
        .collect::<Vec<_>>();
    main_pb.finish_with_message("All hosts have been processed");

    // Print the GPU info
    for (host, info) in infos {
        println!("{}:\n\t{}", host, info.trim().replace('\n', "\n\t"));
    }
    Ok(())
}
