use bytes::Bytes;


async fn make_request(url: &str) -> Result<Bytes, anyhow::Error> {
      
    let _response = match reqwest::get(url).await {
        reqwest::Result::Ok(_response) => {
            log::info!("Downloading .csv file");
            let body: bytes::Bytes = _response.bytes().await?;
            return Ok(body); 
        }
        
        Err(fault) => {
            let error: anyhow::Error = fault.into(); 
            log::error!("Something went wrong with the download {}", error);
            return Err(error);
        }
    };
}


pub async fn download_raw_data() -> anyhow::Result<()> {
    let url: &str = "https://raw.githubusercontent/selva86/datasets/master/BostonHousing.csv";       
    let file_path: String = "/data/bosting_housing.csv".to_string();
    let payload = make_request(url).await; 
    
    log::info!("Saving data to disk");
    std::fs::write(file_path, payload.as_ref());
    Ok(())
 }

