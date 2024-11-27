pub async fn download_raw_data(url: &str) -> Result<bytes::Bytes, anyhow::Error> {

    let _response = match reqwest::blocking::get(url) {
        reqwest::Result::Ok(_response) => { 
            let file_path: String = "/data/bosting_housing.csv".to_string(); 
            let body: bytes::Bytes = _response.bytes()?;
            
            if let Err(e) = std::fs::write(file_path, body.as_ref()) {
               log::error!("Unable to write the file to the file system {}", e); 
            }

            else {
                log::info!("Saved data to disk");
            }

            return Ok(body);
        }
        
        Err(fault) => {
            let error: anyhow::Error = fault.into(); 
            log::error!("Something went wrong with the download {}", error); 
            return Err(error);
        }
    };
}

