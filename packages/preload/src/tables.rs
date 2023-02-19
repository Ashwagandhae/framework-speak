use anyhow::Result;
use csv::ReaderBuilder;
use std::collections::HashMap;

type PhonemePositions<T> = HashMap<String, (T, T)>;
type Conversion = HashMap<String, Vec<String>>;

fn get_phoneme_positions(path: &str, div: usize) -> Result<PhonemePositions<usize>> {
    Ok(ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?
        .into_records()
        // skip header
        .skip(1)
        .enumerate()
        .flat_map(|(y, record)| {
            record
                // can trust the record because I have control over tables made
                .unwrap()
                .into_iter()
                // skip header
                .skip(1)
                .enumerate()
                .flat_map(|(x, cell)| {
                    cell.split_whitespace()
                        .map(|s| (s.to_owned(), (x, y / div)))
                        .collect::<Vec<(String, (usize, usize))>>()
                })
                .collect::<Vec<(String, (usize, usize))>>()
        })
        .collect())
}
fn get_conversion(path: &str) -> Result<Conversion> {
    Ok(ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?
        .into_records()
        .map(|record| {
            let record = record.unwrap();
            (
                record.get(0).unwrap().to_owned(),
                record
                    .get(1)
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.to_owned())
                    .collect(),
            )
        })
        .collect())
}
fn phoneme_to_arpabet_positions(
    phoneme_positions: PhonemePositions<usize>,
    arpabet_to_phoneme: Conversion,
) -> PhonemePositions<usize> {
    arpabet_to_phoneme
        .into_iter()
        .map(|(arpabet, phonemes)| {
            let sum_pos = phonemes
                .iter()
                .map(|phoneme| phoneme_positions.get(phoneme).unwrap())
                // average the positions if there are multiple phonemes (such as aɪ)
                .fold((0, 0), |acc, pos| (acc.0 + pos.0, acc.1 + pos.1));
            let average_pos = (sum_pos.0 / phonemes.len(), sum_pos.1 / phonemes.len());
            (arpabet, average_pos)
        })
        .collect()
}
fn normalize_phoneme_positions(
    phoneme_positions: PhonemePositions<usize>,
) -> PhonemePositions<f32> {
    // put all vals between 0 and 1
    let x_vals = phoneme_positions.iter().map(|(_, pos)| pos.0);
    let y_vals = phoneme_positions.iter().map(|(_, pos)| pos.1);
    let x_min = x_vals.clone().min().unwrap();
    let x_range = x_vals.max().unwrap() - x_min;
    let y_min = y_vals.clone().min().unwrap();
    let y_range = y_vals.max().unwrap() - y_min;
    phoneme_positions
        .into_iter()
        .map(|(phoneme, pos)| {
            (
                phoneme,
                (
                    (pos.0 - x_min) as f32 / x_range as f32,
                    (pos.1 - y_min) as f32 / y_range as f32,
                ),
            )
        })
        .collect()
}
fn squish_phoneme_positions(
    phoneme_positions: PhonemePositions<f32>,
    squish_factor: f32,
) -> PhonemePositions<f32> {
    phoneme_positions
        .into_iter()
        .map(|(phoneme, (x, y))| {
            // x goes from no squish at y = 0 to squish factor at y = 1
            // and is aligned to the right
            let squish = 1.0 + (squish_factor - 1.0) * y;
            (phoneme, ((1.0 - squish) + x * squish, y))
        })
        .collect()
}

// function to generalize for consonants and vowels
fn save_arpabet_pronounce(
    pos_path: &str,
    conversion_path: &str,
    div: usize,
    out_path: String,
    squish_factor: f32,
) -> Result<()> {
    let phoneme_positions = get_phoneme_positions(pos_path, div)?;
    let arpabet_to_phoneme = get_conversion(conversion_path)?;
    let arpabet_positions = phoneme_to_arpabet_positions(phoneme_positions, arpabet_to_phoneme);
    let arpabet_positions = normalize_phoneme_positions(arpabet_positions);
    let arpabet_positions = squish_phoneme_positions(arpabet_positions, squish_factor);

    // save to csv
    let mut wtr = csv::Writer::from_path(out_path).unwrap();
    for (arpabet, pos) in arpabet_positions {
        wtr.write_record(&[arpabet, pos.0.to_string(), pos.1.to_string()])
            .unwrap();
    }

    Ok(())
}
pub fn get_arpabet_pronounce(out_path: &str) -> Result<()> {
    save_arpabet_pronounce(
        "./tables/vowels.csv",
        "./tables/arpabet-vowels.csv",
        1,
        format!("{}/vowels.csv", out_path),
        0.5,
    )?;
    save_arpabet_pronounce(
        "./tables/consonants.csv",
        "./tables/arpabet-consonants.csv",
        2,
        format!("{}/consonants.csv", out_path),
        1.0,
    )?;
    Ok(())
}
