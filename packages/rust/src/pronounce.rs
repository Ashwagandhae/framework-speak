use itertools::Itertools;
use radix_trie::Trie;
use std::collections::HashMap;

mod distance;
use distance::{consonant_distance, silent_consonant_distance, vowel_distance};

pub struct Pronounce {
    phones: Vec<Phone>,
    syllables: u32,
}
impl Pronounce {
    pub fn new(phones: Vec<Phone>) -> Pronounce {
        let syllables = phones.iter().filter(|phone| phone.is_vowel()).count() as u32;
        Pronounce { phones, syllables }
    }
    pub fn empty() -> Pronounce {
        Pronounce {
            phones: Vec::new(),
            syllables: 0,
        }
    }
    // adds a silent consonant between any two vowels that aren't separated by a consonant,
    // and adds a silent consonant at start and end if start/end is vowel, to make it easier to compare
    fn to_normal(&self) -> Self {
        let mut normal_phones: Vec<Phone> = self
            .phones
            .clone()
            .into_iter()
            // add silent consonant between any two vowels that aren't separated by a consonant
            .group_by(|phone| phone.is_vowel())
            .into_iter()
            .flat_map(|(is_vowel, group)| {
                if is_vowel {
                    Itertools::intersperse_with(group, || Phone::Silent).collect::<Vec<_>>()
                } else {
                    group.collect::<Vec<_>>()
                }
            })
            .collect();

        // add silent consonant at start and end if start/end is vowel
        if normal_phones[0].is_vowel() {
            normal_phones.insert(0, Phone::Silent);
        }
        if normal_phones
            .last()
            .map(|phone| phone.is_vowel())
            .unwrap_or(false)
        {
            normal_phones.push(Phone::Silent);
        }
        // syllables should be the same as before
        Self {
            phones: normal_phones,
            syllables: self.syllables,
        }
    }
    fn get_grouped(&self) -> Vec<Vec<Phone>> {
        self.phones
            .iter()
            .cloned()
            .group_by(|phone| phone.is_vowel())
            .into_iter()
            .map(|(_, group)| group.collect::<Vec<_>>())
            .collect()
    }
    // finds the distance of every vowel/consonant pair in the two pronunciations
    // and sums them up
    pub fn distance(&self, other: &Pronounce) -> f32 {
        if self.syllables != other.syllables || self.syllables == 0 {
            return f32::MAX;
        }
        let parts = self.to_normal().get_grouped();
        let other_parts = other.to_normal().get_grouped();

        parts
            .iter()
            .zip(other_parts.iter())
            .map(|(group, other_group)| {
                // go from first to last phoneme in each group at the same speed, comparing each pair
                let total_index = (group.len() * other_group.len()).clamp(0, 32);
                let mut len_ratio = group.len() as f32 / other_group.len() as f32;
                if len_ratio < 1.0 {
                    len_ratio = 1.0 / len_ratio;
                }

                (0..total_index)
                    .map(|i| {
                        let phoneme = &group[i % group.len()];
                        let other_phoneme = &other_group[i % other_group.len()];
                        phoneme.distance(other_phoneme)
                    })
                    .sum::<f32>()
                    / total_index as f32
                    * len_ratio
            })
            .sum::<f32>()
    }
}

#[derive(Debug, Clone)]
pub enum Phone {
    Vowel(f32, f32),
    Consonant(f32, f32),
    Silent,
}
impl Phone {
    fn distance(&self, other: &Phone) -> f32 {
        match (self, other) {
            (Phone::Vowel(x_1, y_1), Phone::Vowel(x_2, y_2)) => vowel_distance(x_1, x_2, y_1, y_2),
            (Phone::Consonant(x_1, y_1), Phone::Consonant(x_2, y_2)) => {
                consonant_distance(x_1, x_2, y_1, y_2)
            }
            (Phone::Silent, Phone::Silent) => 0.0,
            (Phone::Silent, Phone::Consonant(_, y)) | (Phone::Consonant(_, y), Phone::Silent) => {
                silent_consonant_distance(y)
            }
            _ => panic!("tried to compare vowel and consonant"),
        }
    }

    fn is_vowel(&self) -> bool {
        matches!(self, Phone::Vowel { .. })
    }
}
mod cmudict_data;
use cmudict_data::read_cmudict;
mod pronounce_data;
use pronounce_data::read_pronounce;

pub struct PronounceBuilder {
    dict: Trie<String, String>,
    vowel_map: HashMap<String, (f32, f32)>,
    consonant_map: HashMap<String, (f32, f32)>,
}
impl PronounceBuilder {
    pub fn new() -> PronounceBuilder {
        let (vowel_map, consonant_map) = read_pronounce();
        PronounceBuilder {
            dict: read_cmudict(),
            vowel_map,
            consonant_map,
        }
    }
    fn phone_from_arpabet(&self, arpabet: &str) -> Phone {
        self.vowel_map
            .get(arpabet)
            .map(|(x, y)| Phone::Vowel(*x, *y))
            .or_else(|| {
                self.consonant_map
                    .get(arpabet)
                    .map(|(x, y)| Phone::Consonant(*x, *y))
            })
            .unwrap_or_else(|| {
                panic!("Could not find phone for arpabet: {}", arpabet);
            })
    }
    pub fn pronounce_from_arpabet(&self, arpabet: &str) -> Pronounce {
        let phones = arpabet
            .split_whitespace()
            // we don't care about stress
            .map(|part| part.trim_end_matches(|c: char| !c.is_ascii_alphabetic()))
            .map(|part| self.phone_from_arpabet(part))
            .collect();

        Pronounce::new(phones)
    }
    pub fn pronounce(&self, word: &str) -> Pronounce {
        self.dict
            .get(&word.to_ascii_uppercase())
            .map(|s| s.to_string())
            .map(|arpabet| self.pronounce_from_arpabet(&arpabet))
            .unwrap_or_else(Pronounce::empty)
    }
}
