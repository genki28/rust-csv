use csv::Error;
use csv::Reader;
use csv::StringRecord;
use std::fs::File;

pub fn parse(mut reader: Reader<File>) -> Result<Vec<StringRecord>, Error> {
  let mut records: Vec<StringRecord> = vec![];
  for record in reader.records() {
    let record = record?;
    records.push(record)
  }

  Ok(records)
}
