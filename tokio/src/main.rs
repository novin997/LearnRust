use std::io::Error;

use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the runtime
    let rt = Runtime::new()?;

    // Spawn the main task thread
    let result = rt.block_on(async {
        let _listener = TcpListener::bind("127.0.0.1:8080").await?;
        let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
        stream.write_all(b"hello world!").await?;
        println!("test1");
        Ok::<(), Error>(())
    });
    Ok(result.unwrap())
}
