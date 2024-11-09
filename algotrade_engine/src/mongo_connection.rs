use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

use chrono::{Datelike, LocalResult, NaiveDate};
use chrono_tz::America::New_York;
use serde::{Deserialize, Serialize};

const MONGO_CONNECTION_URI: &str = "mongodb://localhost:27017/";
const MONGO_DATABASE_NAME: &str = "GBP_price_history";
const MONGO_COLLECTION_NAME: &str = "1h";

pub async fn mongo_connectivity() -> mongodb::error::Result<()> {
    let uri = MONGO_CONNECTION_URI;

    let client = Client::with_uri_str(uri).await?;

    let database = client.database(MONGO_DATABASE_NAME);
    let collection: Collection<Document> = database.collection(MONGO_COLLECTION_NAME);

    let get_document = collection
        .find_one(doc! {"Date" : "10/30/2024 00:00"}, None)
        .await?;
    println!("Found the doc with date:\n{:#?}", get_document);
    test(&get_document.unwrap()).await;
    Ok(())
}

#[derive(Debug, Default, Deserialize)]
struct PriceRecord {
    day_of_week: String,
    date: String,
    time: String,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
}
struct PriceSegment<Timeframe> {
    begin: String,
    end: String,
    record_list: Vec<PriceRecord>,
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

pub async fn test(input_document: &Document) {
    let date_str = "10/29/2024";
    let date_format = "%m/%d/%Y";
    let date = NaiveDate::parse_from_str(date_str, date_format)
        .expect("Fail to format the date string input");
    println!(
        "Day of the week is {:?} from date {}",
        date.weekday(),
        date_str
    );
    input_document
        .iter()
        .map(|(_, val)| {
            //place(element);
            println!("{key:#?}")
        })
        .collect()
}

fn place(i: u32) {}
