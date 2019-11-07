use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde_derive::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    build: String,
    build_number: u16,
    depends: Vec<String>,
    md5: String,
    name: String,
    sha256: String,
    size: u64,
    timestamp: u64,
    version: String
}

#[derive(Serialize, Deserialize, Debug)]
struct RepodataInfo {
    subdir: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repodata {
    info: RepodataInfo,
    packages: HashMap<String, Record>,
    #[serde(rename = "packages.conda")]
    packages_conda: HashMap<String, Record>,
    repodata_version: u8,
    removed: Vec<String>,
}

pub fn read_repodata<P: AsRef<Path>>(path: P) -> Result<Repodata> {
    // Open the file in read-only mode with buffer.
    let f = File::open(path);
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };
    let reader = BufReader::new(f);

    // Read the JSON contents of the file as an instance of `Repodata`.
    let r = serde_json::from_reader(reader)?;

    // Return the `Repodata`.
    Ok(r)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn test_load_repodata() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests/data/current_repodata.json");
        println!("{}", d.display());
        let _u: Repodata = read_repodata(d).unwrap();
        assert_eq!(_u.info.subdir, "win-64");
    }
}
