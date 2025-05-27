use std::fs::File;
use std::io::Read;

async fn sub(file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://etl-test-higuchi.s3.ap-northeast-1.amazonaws.com/rust-upload/test.txt?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIA4QNCPZ6H5EBMU6XN%2F20250526%2Fap-northeast-1%2Fs3%2Faws4_request&X-Amz-Date=20250526T105252Z&X-Amz-Expires=3600&X-Amz-SignedHeaders=host&X-Amz-Signature=fca268dc5a16e6a7b8e3ef3a2eabd669c6d1f0fdd50736444a520712de9282a5";
    let client = reqwest::Client::new();
    let response = client.put(url)
    println!("{:?}", response);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // sub().await?;

    let mut file = File::open("./test.txt")?;
    
    // println!("{:?}", buf);

    Ok(())
}
