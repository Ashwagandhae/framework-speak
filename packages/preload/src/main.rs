mod cmudict;
mod frameworks;
mod tables;

use cmudict::get_cmudict;
use frameworks::get_frameworks;
use tables::get_arpabet_pronounce;

use anyhow::Result;

fn main() -> Result<()> {
    let cmu_trie = get_cmudict("./output/cmudict.gz")?;
    get_frameworks("./output/frameworks.csv", cmu_trie)?;
    get_arpabet_pronounce("./output/pronounce")?;
    Ok(())
}
