use app;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    match app::execute().await {
        Ok(_) => println!("Success"),
        Err(e) => println!("App Error: {}", e),
    }

    Ok(())
}
