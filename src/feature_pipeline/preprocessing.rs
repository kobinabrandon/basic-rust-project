use polars::{self, error::PolarsResult, frame::DataFrame, io::SerReader, prelude::CsvReadOptions};

pub fn load_csv(path: &str) -> PolarsResult<DataFrame> { 

    let data: DataFrame = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()?;

    Ok(data)
}

