use std::collections::HashSet;
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

use anyhow::Result;

use promptuity::prompts::{Confirm, Input, MultiSelect, MultiSelectOption, Select, SelectOption};
use promptuity::themes::FancyTheme;
use promptuity::{Promptuity, Term};
use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = "./data/config.json";

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Issue {
    InvalidColor,
    EmptyColor,
    InvalidPref,
}

use std::fmt::Display;

use clap::ValueEnum;

#[derive(ValueEnum, Debug, Clone, Default, Serialize, Deserialize)]
pub enum Resolution {
    Ultra,
    High,

    #[default]
    Mid,

    Low,
}

impl Resolution {
    pub fn as_size(&self) -> u32 {
        match self {
            Self::Ultra => 1200,
            Self::High => 1024,
            Self::Mid => 512,
            Self::Low => 256,
        }
    }
}

impl Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Ultra => "ultra",
            Self::High => "high",
            Self::Mid => "mid",
            Self::Low => "low",
        };

        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub input_path: PathBuf,
    pub play_notification_sound: bool,
    pub resolution: Resolution,
    pub ignore_issues: HashSet<Issue>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_path: PathBuf::default(),
            play_notification_sound: true,
            resolution: Resolution::default(),
            ignore_issues: HashSet::default(),
        }
    }
}

impl Config {
    pub fn wizard(need_advanced: bool, only_reqired: bool) -> Result<Self> {
        let res = wizard_(need_advanced, only_reqired);
        if let Err(e) = &res {
            let promptuity_error = e.downcast_ref();

            if let Some(promptuity::Error::Cancel) = promptuity_error {
                exit(0)
            }
        };

        res
    }

    pub fn from_file() -> Result<Self> {
        let reader = BufReader::new(File::open(CONFIG_PATH)?);
        Ok(serde_json::from_reader(reader)?)
    }

    pub fn exist_config_file() -> bool {
        Path::new(CONFIG_PATH).exists()
    }
}

fn wizard_(need_advanced: bool, only_reqired: bool) -> Result<Config> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    let mut config = Config::default();

    p.term().clear()?;

    p.with_intro("おやっさんの作業場").begin()?;

    config.input_path = {
        let raw_path = p
            .prompt(
                Input::new("おやっさん「JSONファイルの場所を教えてくれ。」").with_validator(
                    |value: &String| {
                        let raw_path = value.to_string();
                        let path = Path::new(raw_path.as_str());

                        if !path.exists() {
                            return Err(String::from("no exist path"));
                        }
                        let ext = path.extension().map(|p| p.to_os_string());
                        if ext != Some(OsString::from("json")) {
                            return Err(String::from("not json file."));
                        }

                        Ok(())
                    },
                ),
            )
            .expect("failed to ask json.");

        PathBuf::from(raw_path)
    };

    if only_reqired {
        p.with_outro("おし、じゃあやっていくか。").finish()?;
        return Ok(config);
    }

    if need_advanced {
        p.info("おやっさん「上級者向け設定をしていくぞ。」")?;
        config.resolution = p.prompt(
            Select::new(
                "おやっさん「画質はどうしたいんだ？高いほど時間はかかるぞ。」",
                vec![
                    SelectOption::new(
                        format!("めっちゃ高い ({0:}x{0:})", Resolution::Ultra.as_size()),
                        Resolution::Low,
                    ),
                    SelectOption::new(
                        format!("高め ({0:}x{0:})", Resolution::High.as_size()),
                        Resolution::Low,
                    )
                    .with_hint("デフォルト設定はこれだな。"),
                    SelectOption::new(
                        format!("中くらい ({0:}x{0:})", Resolution::Mid.as_size()),
                        Resolution::Low,
                    ),
                    SelectOption::new(
                        format!("低め ({0:}x{0:})", Resolution::Low.as_size()),
                        Resolution::Low,
                    ),
                ],
            )
            .with_hint("エンターキーかスペースキーで決定できるぞ。"),
        )?;

        config.play_notification_sound = p.prompt(
            Confirm::new("おやっさん「作り終わったときに音を鳴らすか？」").with_default(true),
        )?;

        let ignore_issues = p.prompt(
            MultiSelect::new(
                "おやっさん「無視して注意だけで進めていいことはあるか？」",
                vec![
                    MultiSelectOption::new("誤った形式の色", Some(Issue::InvalidColor)),
                    MultiSelectOption::new("誤った形式の都道府県", Some(Issue::InvalidPref)),
                    MultiSelectOption::new("色の未指定", Some(Issue::EmptyColor))
                        .with_hint("指定しなかったら、「#f00,#0f0,#00f」が使われるぞ。"),
                ],
            )
            .with_hint("スペースキーで選んで、エンターで確定だぞ。"),
        )?;

        config.ignore_issues = ignore_issues
            .into_iter()
            .map(|issue| issue.unwrap())
            .collect::<HashSet<_>>();
    }

    let is_save_config = p.prompt(
        Confirm::new("おやっさん「今回のことは覚えといたほうがいいか？」").with_default(false),
    )?;

    if is_save_config {
        let mut file = File::create(Path::new(CONFIG_PATH))?;
        let json = serde_json::to_string(&config)?;

        file.write_all(json.as_bytes())?;
        p.success("おやっさん「よし、保存しといたから次回も同じ設定で行きたいなら`-u`をつけて実行してみてくれ。」")?;
    }

    p.with_outro("おし、じゃあやっていくか。").finish()?;

    Ok(config)
}
