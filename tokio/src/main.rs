use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the runtime
    let rt = Runtime::new()?;
    loop {}
    // Ok(())
}
