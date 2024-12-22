use anyhow::Ok;
use rand::{prelude::thread_rng, rngs::ThreadRng, seq::SliceRandom};  
use polars::{self, frame::DataFrame, datatypes::{UInt32Chunked, UInt32Type}, chunked_array::ChunkedArray, prelude::{CsvReadOptions, SerReader}};


pub fn load_csv(path: &str) -> Result<DataFrame, anyhow::Error> { 

    let data: DataFrame = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()?;

    Ok(data)
}


pub fn train_test_split(data: &DataFrame, test_ratio: f64) -> anyhow::Result<(DataFrame, DataFrame)> {
      
    let num_rows: usize = data.height();
    let split_index = determine_split_index(&test_ratio, num_rows);
    
    let mut indices: Vec<usize> = (0..num_rows).collect();
    let mut rng: ThreadRng = thread_rng();  
    indices.shuffle(&mut rng);

    let prepped_train_indices: Vec<u32> = change_vector_type(&indices[0..split_index].to_vec());
    let prepped_test_indices: Vec<u32> = change_vector_type(&indices[split_index..].to_vec());    

    let train_indices = make_indices_compatible_with_polars(prepped_train_indices); 
    let test_indices = make_indices_compatible_with_polars(prepped_test_indices); 

    let train_data: DataFrame = data.take(&train_indices)?;
    let test_data: DataFrame = data.take(&test_indices)?;

    Ok((train_data, test_data))
}


pub fn make_features_and_target(data: &DataFrame) -> anyhow::Result<(DataFrame, DataFrame)> {

    let mut feature_names = Vec::new();
    let mut target_name = Vec::new();

    for name in data.get_column_names() {

        if name != "medv" {
            feature_names.push(name.to_string());
        }

        else {
            target_name.push(name.to_string());
        }

    }

    let features = data.select(&feature_names)?;
    let target = data.select(&target_name)?;    

    Ok((features, target))
}



fn determine_split_index(test_ratio: &f64, num_rows: usize) -> usize {
    (num_rows as f64 * test_ratio).ceil() as usize
}


fn change_vector_type(vector: &Vec<usize>) -> Vec<u32> {
   
    // Use the map method of an iterator to execute a closure that changes the type of each element of the iterator through destructuring
    vector.iter().map(|&idx: &usize| idx as u32).collect::<Vec<u32>>()
}


fn make_indices_compatible_with_polars(indices: Vec<u32>) -> ChunkedArray<UInt32Type> {
        
   UInt32Chunked::from_vec("".into(), indices) 

}
