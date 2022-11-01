use std::{fs::File, path::Path};

use clap::Parser;
use parquet::file::{reader::FileReader, serialized_reader::SerializedFileReader};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[clap(author, version = "0.0.1", about = "cli - parquet metadata explorer")]
struct Cli {
    /// File Name.
    #[clap(short, long, default_value = "datasets/data.parquet")]
    filename: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let path = Path::new(&args.filename);

    let file = File::open(&path)?;
    let reader = SerializedFileReader::new(file)?;

    let metadata = reader.metadata();
    let fields = metadata.file_metadata().schema().get_fields();

    for (i, column) in fields.iter().enumerate() {
        let name = column.name();
        let data_type = {
            match column.get_physical_type() {
                parquet::basic::Type::BOOLEAN => "bool",
                parquet::basic::Type::INT32 => "i32",
                parquet::basic::Type::INT64 => "i63",
                parquet::basic::Type::INT96 => "u96",
                parquet::basic::Type::FLOAT => "f32",
                parquet::basic::Type::DOUBLE => "f64",
                parquet::basic::Type::BYTE_ARRAY => "String",
                parquet::basic::Type::FIXED_LEN_BYTE_ARRAY => "String",
            }
        };

        let column_stat = metadata
            .row_group(0)
            .column(i)
            .statistics()
            .ok_or(format!("statistics not found for column name: {}", name))?;

        println!(
            "{} | Name: {} | Data Type: {} | Stats: {:?} ",
            i, name, data_type, column_stat
        );
    }
    Ok(())
}
