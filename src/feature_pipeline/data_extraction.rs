use log::{info, error};
use crate::setup::paths::make_directories;


pub async fn download_raw_data(url: &str, file_path: &str) -> Result<bytes::Bytes, anyhow::Error> {

    simple_logger::init().unwrap();
    make_directories(); 
        
    let _response = match reqwest::get(url).await {
        reqwest::Result::Ok(_response) => { 
            let body: bytes::Bytes = _response.bytes().await?;
            
            if let Err(e) = std::fs::write(file_path, body.as_ref()) {
               error!("Unable to write the file to the file system {}", e); 
            }

            else {
                info!("Saved data to disk");
            }

            return Ok(body);
        }
        
        Err(fault) => {
            let error: anyhow::Error = fault.into(); 
            error!("Something went wrong with the download {}", error); 
            return Err(error);
        }
    };
}

