use clap::Parser;
use rust_barrier::{event::Event, network::NetworkConnection, platform::x11::X11Platform};
use tokio::net::{TcpListener, TcpStream};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Run in server mode
    #[arg(short, long)]
    server: bool,
    
    /// Target server IP
    #[arg(short, long, default_value = "127.0.0.1")]
    ip: String,
    
    /// Port number
    #[arg(short, long, default_value = "8080")]
    port: u16,
    
    /// Virtual display number
    #[arg(long, default_value = "0")]
    display: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    std::env::set_var("DISPLAY", format!(":{}", args.display));

    if args.server {
        run_server(args).await
    } else {
        run_client(args).await
    }
}

async fn run_server(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let platform = X11Platform::new()?;
    platform.grab_input()?;
    
    let listener = TcpListener::bind(format!("{}:{}", args.ip, args.port)).await?;
    println!("Server listening on {}:{}", args.ip, args.port);

    loop {
        let (stream, _) = listener.accept().await?;
        let mut conn = NetworkConnection::new(stream);
        
        platform.run_event_loop(move |event| {
            conn.send_event(event).await.unwrap();
        })?;
    }
}

async fn run_client(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect(format!("{}:{}", args.ip, args.port)).await?;
    let mut conn = NetworkConnection::new(stream);
    let platform = X11Platform::new()?;

    while let Ok(event) = conn.receive_event().await {
        platform.simulate_event(&event)?;
    }
    
    Ok(())
} 