mod data_extraction; 
use crate::data_extraction::download_raw_data;


fn main()  { 
    let url: &str = "https://raw.githubusercontent.com/selva86/datasets/master/BostonHousing.csv";   
    let _data = download_raw_data(url);
}

