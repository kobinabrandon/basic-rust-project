mod feature_pipeline {
    pub mod data_extraction;
    pub mod preprocessing;
}

mod setup {
    pub mod paths;
}

use polars::frame::DataFrame;
use crate::feature_pipeline::preprocessing::load_csv;
use crate::feature_pipeline::data_extraction::download_raw_data;


#[tokio::main]
async fn main() { 
    let url: &str = "https://raw.githubusercontent.com/selva86/datasets/master/BostonHousing.csv";   
    let file_path: String = "data/boston_housing.csv".to_string(); 
    let _ = download_raw_data(url, &file_path).await;

    let data: DataFrame = load_csv(&file_path).expect("Could not load dataframe");
    println!("{:?}", data.head(Some(5)));
}


