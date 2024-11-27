use data_extraction; 



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    download_raw_data().await
}

