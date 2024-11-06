use chrono::{NaiveDate, NaiveDateTime, NaiveWeek};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    process::{self, exit},
    u128,
};

use csv::{Reader, ReaderBuilder, StringRecord};
use std::fs::File;

const CSV_PATH: &str = "./data/GBPUSD_historical_data.csv";

fn main() {
    let x: Record = Record::default();
    x.read_csv(CSV_PATH).;
}

#[derive(Debug, Default, Deserialize)]
struct Record {
    day_of_week: String,
    date: String,
    time: String,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
}

impl Record {
    pub fn read_csv(&self, path: &str) -> Result<Vec<StringRecord>, Box<dyn Error>> {
        let csv_file = File::open(path)?;
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(csv_file);
        let mut res_vec = Vec::new();
        for result in reader.records() {
            match result {
                Ok(_) => {
                    let record = result?;
                    //println!(
                    //    "{} {} {} {} {}",
                    //    &record[0], &record[1], &record[2], &record[3], &record[4],
                    //);
                    res_vec.push(record);
                }
                Err(err) => {
                    println!("Error parsing data {}", err);
                    process::exit(1)
                }
            }
        }
        for element in res_vec.iter() {
            println!("{element:?}")
        }
        Ok(res_vec)
    }
    pub fn transform_from_raw(self) -> Vec<Self> {
        vec![]
    }
}

struct PriceSegment<Timeframe> {
    begin: String,
    end: String,
    record_list: Vec<Record>,
    time_frame: Timeframe,
}

enum Timeframe {
    OneMin,
    FiveMin,
    FifteenMin,
    Onehour,
    Fourhour,
    Daily,
    Weekly,
    Monthly,
}

impl PriceSegment<Timeframe> {
    pub fn extract_into_weekll_segment(&self) {}
}
