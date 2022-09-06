use std::{error::Error, fs::File, io::BufReader, path::Path};
type WordleDictionary = Vec<String>;

pub fn read_dictionary_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<WordleDictionary, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let json = serde_json::from_reader(reader)?;
    Ok(json)
}
