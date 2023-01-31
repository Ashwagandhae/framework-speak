use csv::ReaderBuilder;

static WORDS: &str = include_str!("../../preload/output/frameworks.csv");

pub fn read_frameworks() -> Vec<(String, String)> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(WORDS.as_bytes());
    reader
        .records()
        .map(|record| {
            let record = record.unwrap();
            let key = record.get(0).unwrap();
            let value = record.get(1).unwrap();
            (key.to_owned(), value.to_owned())
        })
        .collect()
}
