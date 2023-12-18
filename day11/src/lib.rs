// day11/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 11 Challenge Endpoints

// dependencies
use common_features::{WebRequest, PngImage};
use std::convert::Infallible;
use std::fs;

const FILE: &str = "/home/jeff/dev/source/repos/rust/shuttle-cch23/day11/assets/decoration.png";

// endpoint handler to return a static image of Christmas decorations
pub async fn svc_static_files(_request: WebRequest) -> Result<PngImage, Infallible> {
    
    let metadata = fs::metadata(FILE).unwrap();
    let byte_length = metadata.len();

    let png_image = PngImage { size: byte_length };

    Ok(png_image)
}
