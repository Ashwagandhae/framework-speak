use csv::ReaderBuilder;
use std::collections::HashMap;

static VOWELS: &str = include_str!("../../../preload/output/pronounce/vowels.csv");
static CONSONTANTS: &str = include_str!("../../../preload/output/pronounce/consonants.csv");

type PhonemeMap = HashMap<String, (f32, f32)>;
fn get_phoneme_map(csv_string: &str) -> PhonemeMap {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv_string.as_bytes());
    reader
        .records()
        .map(|record| {
            let record = record.unwrap();
            let key = record.get(0).unwrap();
            let x = record.get(1).unwrap().parse::<f32>().unwrap();
            let y = record.get(2).unwrap().parse::<f32>().unwrap();
            (key.to_owned(), (x, y))
        })
        .collect()
}
pub fn read_pronounce() -> (PhonemeMap, PhonemeMap) {
    let vowel_map = get_phoneme_map(VOWELS);
    let consonant_map = get_phoneme_map(CONSONTANTS);
    (vowel_map, consonant_map)
}
