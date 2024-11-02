use serde::{Deserialize, Serialize};

use std::{collections::HashMap, error::Error, process, u128};

use csv::{Reader, ReaderBuilder};
use std::fs::File;

const CSV_PATH: &str = "./data/GBPUSD_historical_data.csv";

#[derive(Debug, Deserialize)]
struct Record {
    date: String,
    time: String,
    open: String,
    high: String,
    low: String,
    close: String,
    change: String,
    change_in_percentage: String,
}

pub fn read_csv() -> Result<(), Box<dyn Error>> {
    let csv_file = File::open(CSV_PATH)?;

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv_file);

    for result in reader.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = read_csv() {
        println!("{}", err);
        process::exit(1)
    }
}
