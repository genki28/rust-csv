extern crate csv;
use csv::Error;

fn main() -> Result<(), Error> {
  let csv = "year,name
                  2020,令和
                  1989,平成
                  1926 昭和
                  1912 大正
                  1897 明治";
  let mut reader = csv::Reader::from_reader(csv.as_bytes());
  for record in reader.records() {
    let record = record?;
    println!(
      "In {}, {} built the {} model. It is a {}.",
      &record[0], &record[1], &record[2], &record[3]
    );
  }

  Ok(())
}
