use clap::Parser;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use futures::stream::{self, StreamExt};
use std::net::SocketAddr;
use std::error::Error;

/// A simple port sniffer.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Target IP address
    #[clap(short = 'T', long, default_value = "127.0.0.1")]
    target: String,

    /// Starting port number
    #[clap(short = 's', long, default_value_t = 1)]
    start_port: u16,

    /// Ending port number
    #[clap(short = 'e', long, default_value_t = 65535)]
    end_port: u16,

    /// Timeout duration in seconds
    #[clap(short = 't', long, default_value_t = 1)]
    timeout: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Validate port range
    if args.start_port > args.end_port {
        eprintln!("Start port must be less than or equal to end port.");
        std::process::exit(1);
    }

    let ports = args.start_port..=args.end_port;

    let stream = stream::iter(ports)
        .map(|port| {
            let addr = match format!("{}:{}", args.target, port).parse::<SocketAddr>() {
                Ok(addr) => addr,
                Err(e) => {
                    eprintln!("Invalid address '{}:{}' - {}", args.target, port, e);
                    return tokio::spawn(async { None });
                }
            };
            tokio::spawn(async move {
                match timeout(Duration::from_secs(args.timeout), TcpStream::connect(addr)).await {
                    Ok(Ok(_)) => Some(port),
                    Ok(Err(e)) => {
                        eprintln!("Failed to connect to {}: {} - {}", addr, port, e);
                        None
                    }
                    Err(e) => {
                        eprintln!("Connection to {}:{} timed out: {}", addr, port, e);
                        None
                    }
                }
            })
        })
        .buffer_unordered(100);

    stream
        .for_each(|res| async move{
            if let Ok(Some(port)) = res {
                println!("Port {} is open", port);
            }
        })
        .await;

    Ok(())
}
