use reqwest::Body;
use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // sub().await?;

    let tmp_url = "";
    let mut file = File::open("./test.txt").await?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await?;

    let client = reqwest::Client::new();
    let res = client.put(tmp_url).body(Body::from(buf)).send().await?;

    if res.status().is_success() {
        println!("File uploaded successfully.");
    } else {
        println!("Failed to upload file: {}", res.status());
    }

    Ok(())
}
