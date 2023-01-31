use flate2::read::GzDecoder;
use radix_trie::Trie;
use std::io::Read;

static DICT: &[u8] = include_bytes!("../../../preload/output/cmudict.gz");

pub fn read_cmudict() -> Trie<String, String> {
    let mut decoder = GzDecoder::new(DICT);
    let mut s = String::new();
    decoder
        .read_to_string(&mut s)
        .expect("Failed to read cmudict");
    s.lines()
        .filter(|line| !line.starts_with(";;;"))
        .map(|line| {
            let mut parts = line.split("  ");
            (
                parts.next().unwrap().to_owned(),
                parts.next().unwrap().to_owned(),
            )
        })
        .collect()
}
