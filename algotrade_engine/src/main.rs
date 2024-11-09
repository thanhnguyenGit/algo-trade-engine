mod mongo_connection;

use chrono::{NaiveDate, NaiveDateTime, NaiveWeek};
use mongo_connection::{mongo_connectivity, test};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    process::{self, exit},
    u128,
};
use tokio::io::join;

use csv::{Reader, ReaderBuilder, StringRecord};

use std::fs::File;
const CSV_PATH: &str = "./data/GBPUSD_historical_data.csv";

#[tokio::main]
async fn main() {
    mongo_connectivity().await;
    //test().await;
}
