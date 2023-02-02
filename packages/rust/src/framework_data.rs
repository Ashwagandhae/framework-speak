use csv::ReaderBuilder;

static WORDS: &str = include_str!("../../preload/output/frameworks.csv");

pub fn read_frameworks() -> Vec<(String, String, u64)> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(WORDS.as_bytes());
    reader
        .records()
        .map(|record| {
            let record = record.unwrap();
            let name = record.get(0).unwrap();
            let pronounce = record.get(1).unwrap();
            let stars = record.get(2).unwrap();
            (
                name.to_owned(),
                pronounce.to_owned(),
                stars.parse::<u64>().unwrap(),
            )
        })
        .collect()
}
