use rust_barrier::platform::x11::X11Platform;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing X11 platform implementation...");
    
    // Create X11 platform
    let platform = X11Platform::new()?;
    println!("✓ X11 connection established");

    // Try to grab input
    platform.grab_input()?;
    println!("✓ Input grabbed successfully");

    // Keep running for a few seconds to test mouse movement
    println!("Move your mouse - watching for 5 seconds...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    println!("Test complete!");
    Ok(())
} 