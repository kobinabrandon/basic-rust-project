mod feature_pipeline {
    pub mod data_extraction;
        mod preprocessing;
}

mod setup {
    pub mod paths;
}

use crate::feature_pipeline::data_extraction::download_raw_data;

#[tokio::main]
async fn main() { 
    let url: &str = "https://raw.githubusercontent.com/selva86/datasets/master/BostonHousing.csv";   
    let _data = download_raw_data(url).await;
}

