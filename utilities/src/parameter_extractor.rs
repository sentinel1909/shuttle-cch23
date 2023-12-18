// utilities/src/path_parser.rs

// dependencies

pub fn parameter_extractor(path: &str) -> Vec<i32> {
    let path_segments: Vec<Result<i32, _>> = path
        .split('/')
        .map(|segment| segment.parse::<i32>())
        .collect();

    let segments: Vec<i32> = path_segments
        .into_iter()
        .filter_map(|path_segment| path_segment.ok())
        .collect();

    segments
}
