use anyhow::Result;
use image::{Pixel, Rgb};
use serde::Deserialize;

use crate::{Pref, PrefDict};

use super::{config::Config, Issue};

#[derive(Debug, Clone, Deserialize)]
pub struct RawJson {
    prefs: Vec<String>,
    colors: Vec<String>,
}

#[derive(Debug, Clone, Hash)]
pub struct InputData {
    pub prefs: Vec<Pref>,
    pub colors: Vec<Rgb<u8>>,
}

impl InputData {
    pub fn from_raw_json(raw_json: RawJson, pref_dict: &PrefDict, config: &Config) -> Self {
        let prefs = sanitize_raw_prefs(raw_json.prefs, pref_dict);

        let say_err = |msg: String, issue: Issue| {
            if config.ignore_issues.contains(&issue) {
                println!("{}", msg);
            } else {
                panic!("{}", msg);
            }
        };

        let prefs = prefs
            .into_iter()
            .filter_map(|maybe_pref| match maybe_pref {
                Ok(pref) => Some(pref),
                Err(raw) => {
                    say_err(
                        format!(
                            "おやっさん「おい！`{}`なんて都道府県、地図にないぞ！」",
                            raw
                        ),
                        Issue::InvalidPref,
                    );

                    None
                }
            })
            .collect::<Vec<_>>();

        let mut colors = sanitize_raw_colors(raw_json.colors)
            .into_iter()
            .filter_map(|c| match c {
                Ok(c) => Some(c),
                Err(raw) => {
                    say_err(
                        format!("おやっさん「おい！`{}`ってどんな色かわかんねぇよ！」", raw),
                        Issue::InvalidColor,
                    );

                    None
                }
            })
            .collect::<Vec<_>>();

        if colors.is_empty() {
            say_err(
                String::from("おやっさん「おい！色は少なくとも１種類指定してくれないと困るぞ。」"),
                Issue::EmptyColor,
            );

            colors = vec![
                *Rgb::<u8>::from_slice(&[255, 0, 0]),
                *Rgb::<u8>::from_slice(&[0, 255, 0]),
                *Rgb::<u8>::from_slice(&[0, 0, 255]),
            ];
        }

        InputData { prefs, colors }
    }
}

fn sanitize_raw_prefs(raw_prefs: Vec<String>, pref_dict: &PrefDict) -> Vec<Result<Pref, String>> {
    raw_prefs
        .into_iter()
        .map(|raw_pref| {
            if let Some(pref) = Pref::new(pref_dict, raw_pref.clone()) {
                Ok(pref)
            } else {
                Err(raw_pref)
            }
        })
        .collect::<Vec<_>>()
}

fn sanitize_raw_colors(raw_colors: Vec<String>) -> Vec<Result<Rgb<u8>, String>> {
    let to_rgb = |c: String| -> Result<Rgb<u8>, String> {
        let color = if c.starts_with('#') {
            if c.len() == 7 {
                [&c[1..3], &c[3..5], &c[5..7]].map(|e| u8::from_str_radix(e, 16))
            } else if c.len() == 4 {
                [&c[1..3], &c[3..5], &c[5..7]].map(|e| u8::from_str_radix(e, 16))
            } else {
                return Err(c);
            }
        } else {
            return Err(c);
        };

        if let [Ok(r), Ok(g), Ok(b)] = color {
            Ok(*Rgb::from_slice(&[r, g, b]))
        } else {
            Err(c)
        }
    };

    raw_colors.into_iter().map(to_rgb).collect::<Vec<_>>()
}

pub fn input(config: &Config) -> Result<InputData> {
    let pref_dict = PrefDict::load_from_csv()?;
    let json_path = config.input_path.clone();

    let raw_json: RawJson = serde_json::from_str(std::fs::read_to_string(json_path)?.as_str())?;
    Ok(InputData::from_raw_json(raw_json, &pref_dict, config))
}
