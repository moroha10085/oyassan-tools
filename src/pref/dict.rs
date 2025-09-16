use std::path::Path;

use anyhow::Result;

const DICT_PATH: &str = "./data/pref.csv";

pub struct Dict(pub Vec<Vec<String>>);

impl Dict {
    pub fn load_from_csv() -> Result<Self> {
        let pref_csv_path = Path::new(DICT_PATH);
        let raw_dict = std::fs::read_to_string(pref_csv_path)?;

        let pref_dict = raw_dict
            .split("\n")
            .filter_map(|row| {
                let trimed_row = row
                    .split(",")
                    .filter_map(|elem| {
                        let trimed_elem = elem.trim();

                        if trimed_elem.is_empty() {
                            None
                        } else {
                            Some(trimed_elem.to_string())
                        }
                    })
                    .collect::<Vec<_>>();

                if trimed_row.is_empty() {
                    None
                } else {
                    Some(trimed_row)
                }
            })
            .collect::<Vec<_>>();

        Ok(Dict(pref_dict))
    }

    pub fn row(&self, idx: usize) -> Option<&[String]> {
        if idx > self.0.len() {
            return None;
        }

        Some(&self.0[idx])
    }

    pub fn key_at(&self, idx: usize) -> Option<&str> {
        self.row(idx).map(|row| row[0].as_str())
    }

    pub fn get(&self, pref_idx: usize, char_type_idx: usize) -> &str {
        &self.0[pref_idx][char_type_idx]
    }
}
