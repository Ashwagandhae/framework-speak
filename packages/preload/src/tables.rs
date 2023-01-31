use anyhow::Result;
use csv::ReaderBuilder;
use std::collections::HashMap;

fn get_phoneme_positions(path: &str, div: usize) -> Result<HashMap<String, (usize, usize)>> {
    let mut reader = ReaderBuilder::new().has_headers(false).from_path(path)?;
    Ok(reader
        .records()
        .enumerate()
        .map(|(y, record)| {
            // can trust the record because I have control over tables made
            record
                .unwrap()
                .iter()
                .enumerate()
                .map(|(x, cell)| {
                    // split cell by /n
                    cell.split_whitespace()
                        .map(|s| (s.to_owned(), (x / div, y / div)))
                        .collect::<Vec<(String, (usize, usize))>>()
                })
                .flatten()
                .collect::<Vec<(String, (usize, usize))>>()
        })
        .flatten()
        .collect())
}
fn get_conversion(path: &str) -> Result<HashMap<String, String>> {
    let mut reader = ReaderBuilder::new().has_headers(false).from_path(path)?;
    Ok(reader
        .records()
        .map(|record| {
            let record = record.unwrap();
            (
                record.get(0).unwrap().to_owned(),
                record.get(1).unwrap().to_owned(),
            )
        })
        .collect())
}
fn phoneme_to_arpabet_positions(
    phoneme_positions: HashMap<String, (usize, usize)>,
    arpabet_to_phoneme: HashMap<String, String>,
) -> HashMap<String, (usize, usize)> {
    arpabet_to_phoneme
        .iter()
        .map(|(arpabet, phoneme)| {
            let pos = phoneme_positions.get(phoneme).unwrap();
            (arpabet.to_owned(), pos.clone())
        })
        .collect()
}
fn normalize_phoneme_positions(
    phoneme_positions: HashMap<String, (usize, usize)>,
) -> HashMap<String, (f32, f32)> {
    // put all vals between 0 and 1
    let x_vals = phoneme_positions.iter().map(|(_, pos)| pos.0);
    let y_vals = phoneme_positions.iter().map(|(_, pos)| pos.1);
    let x_min = x_vals.clone().min().unwrap();
    let x_range = x_vals.max().unwrap() - x_min;
    let y_min = y_vals.clone().min().unwrap();
    let y_range = y_vals.max().unwrap() - y_min;
    phoneme_positions
        .iter()
        .map(|(phoneme, pos)| {
            (
                phoneme.to_owned(),
                (
                    (pos.0 - x_min) as f32 / x_range as f32,
                    (pos.1 - y_min) as f32 / y_range as f32,
                ),
            )
        })
        .collect()
}
// function to generalize for consonants and vowels
fn save_arpabet_pronounce(pos_path: &str, conversion_path: &str, div: usize, out_path: String) {
    let phoneme_positions = get_phoneme_positions(pos_path, div).unwrap();
    let arpabet_to_phoneme = get_conversion(conversion_path).unwrap();
    let arpabet_positions = phoneme_to_arpabet_positions(phoneme_positions, arpabet_to_phoneme);
    let arpabet_positions = normalize_phoneme_positions(arpabet_positions);
    // save to csv
    let mut wtr = csv::Writer::from_path(out_path).unwrap();
    for (arpabet, pos) in arpabet_positions {
        wtr.write_record(&[arpabet, pos.0.to_string(), pos.1.to_string()])
            .unwrap();
    }
}
pub fn get_arpabet_pronounce(out_path: &str) -> Result<()> {
    save_arpabet_pronounce(
        "./tables/vowels.csv",
        "./tables/arpabet-vowels.csv",
        1,
        format!("{}/vowels.csv", out_path),
    );
    save_arpabet_pronounce(
        "./tables/consonants.csv",
        "./tables/arpabet-consonants.csv",
        2,
        format!("{}/consonants.csv", out_path),
    );
    Ok(())
}
