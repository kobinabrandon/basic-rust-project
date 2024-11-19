use std::str::Bytes;
use reqwest::Response;


fn main() -> anyhow::Result<()> {
   download_raw_data() 
}


async fn make_request(url: &str) -> Result<Response, reqwest::Error> {
     
    match reqwest::get(url).await {
        Ok(payload) => {
            log::info!("Got a bite!");
            return Ok(payload);
        },
        Err(fault) => {
            log::error!("Something went wrong with the download {}", fault);
            return Err(fault);
        }
    };

}


fn download_raw_data() -> anyhow::Result<()> {

    let url: &str = "https://raw.githubusercontent/selva86/datasets/master/BostonHousing.csv";       
    log::info!("Downloading .csv file");

    let payload = make_request(url);  
    let bytes: Bytes = payload.bytes()?;    
    
    log::info!("Saving data to disk");
    std::fs::write("/data", bytes);
    Ok(())
 }

