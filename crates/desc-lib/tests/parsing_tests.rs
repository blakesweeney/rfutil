use desc_lib::reader::DescParser;
use desc_lib::to_path;
use std::path::PathBuf;

use pretty_assertions::assert_eq;
use rstest::rstest;
use tempfile::NamedTempFile;

#[rstest]
fn should_parse_all_files(#[files("tests/files/RF*")] filename: PathBuf) {
    let reader = DescParser::from_path(&filename).unwrap();
    let desc_file = reader.parse();
    assert!(desc_file.is_ok());
}

// #[rstest]
// fn can_roundtrip_all_files(#[files("tests/files/RF*")] filename: PathBuf) {
//     let reader = DescParser::from_path(&filename).unwrap();
//     let desc_file = reader.parse();
//     assert!(desc_file.is_ok());
//     let first_result = desc_file.unwrap();
//
//     let output = NamedTempFile::new().unwrap();
//     let r = to_path(output.path(), &first_result);
//     assert!(r.is_ok());
//     println!("{}", std::fs::read_to_string(output.path()).unwrap());
//
//     let text_reader = DescParser::from_path(output.path()).unwrap();
//     let text_result = text_reader.parse();
//     assert!(text_result.is_ok());
//     assert_eq!(first_result, text_result.unwrap());
// }
