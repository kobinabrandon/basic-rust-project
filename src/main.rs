use std::str::Bytes;
use reqwest::Response;


fn main() -> anyhow::Result<()> {
   download_raw_data() 
}

fn download_raw_data() -> anyhow::Result<()> {

    use std::error::Error;
    let url: &str = "https://raw.githubusercontent/selva86/datasets/master/BostonHousing.csv";    
    
    log::info!("Downloading .csv file");

    let response: Result<Response, Error> = reqwest::get(url)?;  
    let bytes: Bytes = response.bytes()?;    
    
    log::info!("Saving data to disk");
    std::fs::write("/data", bytes);
    Ok(())
 }

