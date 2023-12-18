// day11/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 11 Challenge Endpoints

// dependencies
use common_features::{PngImage, WebRequest};
use std::convert::Infallible;
use std::fs;

// a constant which represents the file path
const FILE: &str = "/home/jeff/dev/source/repos/rust/shuttle-cch23/day11/assets/decoration.png";

// endpoint handler to return a static image of Christmas decorations
pub async fn svc_static_files(_request: WebRequest) -> Result<PngImage, Infallible> {
    // get the metadata for the decorations.png file
    let metadata = fs::metadata(FILE).unwrap();

    // get the byte length of the Christmas decorations image
    let byte_length = metadata.len();

    // create an instance of the PngImage type, save the size in its "size" field
    let png_image = PngImage { size: byte_length };

    // return the struct
    Ok(png_image)
}
