mod file;
mod parser;

extern crate csv;
use csv::Error;
use std::env;
use std::fs::File;

fn main() -> Result<(), Error> {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  file::extension_check(filename);
  let f = File::open(filename).expect("file not found");
  let reader = csv::Reader::from_reader(f);
  let res = parser::parse(reader);
  println!("{:?}", res);

  Ok(())
}

fn csv_to_json() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  file::extension_check(filename);
  let f = File::open(filename).expect("file not found");
  let reader = csv::Reader::from_reader(f);
  let res = parser::parse(reader).unwrap();

  let serialized: String = serde_json::to_string(&res);
}
