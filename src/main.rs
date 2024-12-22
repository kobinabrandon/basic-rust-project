mod feature_pipeline {
    pub mod data_extraction;
    pub mod preprocessing;
}

mod training_pipeline {
    pub mod training;
}

mod setup {
    pub mod paths;
}


use polars::frame::DataFrame;
use crate::feature_pipeline::data_extraction::download_raw_data;
use crate::feature_pipeline::preprocessing::{load_csv, train_test_split, make_features_and_target};


#[tokio::main]
async fn main() -> anyhow::Result<()> { 
    let url: &str = "https://raw.githubusercontent.com/selva86/datasets/master/BostonHousing.csv";   
    let file_path: String = "data/boston_housing.csv".to_string(); 
    let _ = download_raw_data(url, &file_path).await;

    let data: DataFrame = load_csv(&file_path).expect("Could not load dataframe");

    let (mut training_data, mut test_data) = train_test_split(&data, 0.2)?;
    let (mut x_train, mut y_train) = make_features_and_target(&training_data)?;
    let (mut x_test, mut y_test) = make_features_and_target(&test_data)?;
        

    Ok(())
}


