use std::str::Bytes;
use reqwest::{Response, Error};


fn main() -> anyhow::Result<()> {
   download_raw_data() 
}


fn make_request(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let response = reqwest::get(url)?;
    Ok(())
}


fn download_raw_data() -> anyhow::Result<()> {

    let url: &str = "https://raw.githubusercontent/selva86/datasets/master/BostonHousing.csv";       
    log::info!("Downloading .csv file");

    let payload = make_request(url=url);  
    let bytes: Bytes = payload.bytes()?;    
    
    log::info!("Saving data to disk");
    std::fs::write("/data", bytes);
    Ok(())
 }

