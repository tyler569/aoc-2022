use std::{fs, path::Path, env};

pub mod parser;
pub mod regexparser;

pub fn get_input(year: usize, day: usize) -> anyhow::Result<String> {
    let cache_file_name = format!("cache/{}/{}", year, day);
    let cache_file = Path::new(&cache_file_name);

    if cache_file.exists() {
        let content = fs::read_to_string(cache_file)?;
        Ok(content)
    } else {
        let key = env::var("AOC_SESSION").expect("No AoC session key found in environment!");
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

        let content = ureq::get(&url).set("Cookie", &key).call()?.into_string()?;
        fs::create_dir_all(cache_file.parent().expect("cache always has a parent"))?;
        fs::write(cache_file, &content)?;

        Ok(content)
    }
}