use std::thread;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the runtime
    let rt = Runtime::new()?;

    // Spawn the main task thread
    rt.block_on(async {
        // let _listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
        loop {
            let sum = expensive_operation();
            println!("{sum}");
            stream.write_all(&sum.to_string().as_bytes()).await.unwrap();
            thread::sleep(Duration::from_millis(2000));
        }
        // Ok::<(), Error>(())
    });
    Ok(())
}

fn expensive_operation() -> i32 {
    let mut val = 0;
    for i in 1..10000 {
        val = val + i;
    }
    val
}
