use std::path::Path;

pub fn extension_check(filename: &str) {
  let result = Path::new(&filename).extension().unwrap();
  if result != "csv" {
    panic!("csv拡張子のファイルを指定してください")
  }
}
