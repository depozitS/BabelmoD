
pub struct RawSegmentData{
    key: String,
    value: Vec<String>,
}

impl RawSegmentData{
    pub fn new(key: &str, value: Vec<&str> ) -> Self{
        RawSegmentData { key: key.to_string(), value: value.into_iter().map(String::from).collect() }
    }
}