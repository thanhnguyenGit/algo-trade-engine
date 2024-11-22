use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

use chrono::{Datelike, LocalResult, NaiveDate};
use chrono_tz::America::New_York;
use serde::{Deserialize, Serialize};

const MONGO_CONNECTION_URI: &str = "mongodb://localhost:27017/";
const MONGO_DATABASE_NAME: &str = "admin";
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
    conver_from_raw(&get_document.unwrap()).await;
    Ok(())
}

#[derive(Debug, Default, Deserialize)]
pub struct MongoRecord {
    pub day_of_week: String,
    pub date: String,
    pub time: String,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
}
pub struct PriceSegment {
    pub begin: String,
    pub end: String,
    pub record_list: Vec<MongoRecord>,
    pub time_frame: Timeframe,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Timeframe {
    OneMin,
    FiveMin,
    FifteenMin,
    Onehour,
    Fourhour,
    Daily,
    Weekly,
    Monthly,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SegIndex {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}
pub async fn conver_from_raw(input_document: &Document) {
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
        .map(|(_, bson)| {
            println!("BSON-DATE {:#?}", bson.as_str());
            println!("BSON-DOUBLE {:#?}", bson.as_f64());
        })
        .collect()
}

impl From<Document> for MongoRecord {
    fn from(value: Document) -> Self {
        let mut day_of_week
        value.iter().map(|(_, bson)| date = bson.as_str().take())
    }
}

trait SegmentBuilder {
    type OutputType;
    fn set_begin(&mut self, begin: String);
    fn set_timeframe(&mut self, timeframe: Timeframe);
    fn set_seg_index(&mut self, index: SegIndex);
    fn set_price_records(&mut self, record: &[MongoRecord]);
    fn set_end(&mut self, end: String);
    fn build(self) -> Self::OutputType;
}
impl SegmentBuilder for PriceSegment {
    type OutputType = PriceSegment;
    fn set_begin(&mut self, begin: String) {}
    fn set_timeframe(&mut self, timeframe: Timeframe) {}
    fn set_seg_index(&mut self, index: SegIndex) {}
    fn set_price_records(&mut self, record: &[MongoRecord]) {}
    fn set_end(&mut self, end: String) {}
    fn build(self) -> Self::OutputType {
        PriceSegment {
            end: Default::default(),
            begin: Default::default(),
            time_frame: Timeframe::Onehour,
            record_list: Default::default(),
        }
    }
}
