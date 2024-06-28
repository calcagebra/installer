use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Release {
    tag_name: String,
    assets_url: String,
}

#[derive(Deserialize, Debug)]
struct Asset {
    browser_download_url: String,
    name: String,
    /// The size of the asset in bytes
    size: u64
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    
    Ok(())
}
