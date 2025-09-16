use crate::PrefDict;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Pref {
    Aichi,
    Akita,
    Aomori,
    Chiba,
    Ehime,
    Fukui,
    Fukuoka,
    Fukushima,
    Gifu,
    Gumma,
    Hiroshima,
    Okayama,
    Hokkai,
    Hyogo,
    Ibaraki,
    Ishikawa,
    Iwate,
    Kagawa,
    Kagoshima,
    Kanagawa,
    Kochi,
    Kumamoto,
    Kyoto,
    Mie,
    Miyagi,
    Miyazaki,
    Nagano,
    Nagasaki,
    Nara,
    Niigata,
    Oita,
    Okinawa,
    Osaka,
    Saga,
    Saitama,
    Shiga,
    Shimane,
    Shizuoka,
    Tochigi,
    Tokushima,
    Tokyo,
    Tottori,
    Toyama,
    Wakayama,
    Yamagata,
    Yamaguchi,
    Yamanashi,
}

impl Pref {
    pub fn new(pref_dict: &PrefDict, raw_pref: String) -> Option<Self> {
        // 正規表現を使わずにゴリ押しでチェック。
        // 前提条件:
        // 文字種は統一されている ex) 京都、あきた、（かナガワとかにはならない）
        // 一文字目から都道府県名が始まっている
        // 末尾からいくつかの文字は都、道、府、県のような要素の可能性がある
        // 先頭からの部分的な一致で判断しても良い ex) 東京都府 -> 東京 (全体で見ればない都道府県だけどok)
        // ↑完全一致とかの厳格なチェックにあまり意味がないので。
        // 辞書の要素を最後まで見たとき、常に一意に定まる

        let mut candidate_idx_list = Vec::new();

        for pref_idx in 0..pref_dict.0.len() {
            for type_idx in 0..4 {
                candidate_idx_list.push((pref_idx, type_idx))
            }
        }

        for (char_pos, pref_char) in raw_pref.chars().enumerate() {
            let idx_list = candidate_idx_list.clone();
            candidate_idx_list.clear();

            for (pref_idx, type_idx) in idx_list {
                let target = &pref_dict.get(pref_idx, type_idx);

                let Some(target_char) = target.chars().nth(char_pos) else {
                    let key = pref_dict.key_at(pref_idx).expect("wrong key idx.");

                    return Self::from_key(key);
                };
                if target_char == pref_char {
                    candidate_idx_list.push((pref_idx, type_idx));
                }
            }
        }

        if candidate_idx_list.len() == 1 {
            let key_idx = candidate_idx_list[0].0;
            let key = pref_dict.key_at(key_idx).expect("wrong key idx.");

            return Self::from_key(key);
        }

        None
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "aichi" => Some(Self::Aichi),
            "akita" => Some(Self::Akita),
            "aomori" => Some(Self::Aomori),
            "chiba" => Some(Self::Chiba),
            "ehime" => Some(Self::Ehime),
            "fukui" => Some(Self::Fukui),
            "fukuoka" => Some(Self::Fukuoka),
            "fukushima" => Some(Self::Fukushima),
            "gifu" => Some(Self::Gifu),
            "gumma" => Some(Self::Gumma),
            "hiroshima" => Some(Self::Hiroshima),
            "okayama" => Some(Self::Okayama),
            "hokkai" => Some(Self::Hokkai),
            "hyogo" => Some(Self::Hyogo),
            "ibaraki" => Some(Self::Ibaraki),
            "ishikawa" => Some(Self::Ishikawa),
            "iwate" => Some(Self::Iwate),
            "kagawa" => Some(Self::Kagawa),
            "kagoshima" => Some(Self::Kagoshima),
            "kanagawa" => Some(Self::Kanagawa),
            "kochi" => Some(Self::Kochi),
            "kumamoto" => Some(Self::Kumamoto),
            "kyoto" => Some(Self::Kyoto),
            "mie" => Some(Self::Mie),
            "miyagi" => Some(Self::Miyagi),
            "miyazaki" => Some(Self::Miyazaki),
            "nagano" => Some(Self::Nagano),
            "nagasaki" => Some(Self::Nagasaki),
            "nara" => Some(Self::Nara),
            "niigata" => Some(Self::Niigata),
            "oita" => Some(Self::Oita),
            "okinawa" => Some(Self::Okinawa),
            "osaka" => Some(Self::Osaka),
            "saga" => Some(Self::Saga),
            "saitama" => Some(Self::Saitama),
            "shiga" => Some(Self::Shiga),
            "shimane" => Some(Self::Shimane),
            "shizuoka" => Some(Self::Shizuoka),
            "tochigi" => Some(Self::Tochigi),
            "tokushima" => Some(Self::Tokushima),
            "tokyo" => Some(Self::Tokyo),
            "tottori" => Some(Self::Tottori),
            "toyama" => Some(Self::Toyama),
            "wakayama" => Some(Self::Wakayama),
            "yamagata" => Some(Self::Yamagata),
            "yamaguchi" => Some(Self::Yamaguchi),
            "yamanashi" => Some(Self::Yamanashi),
            _ => None,
        }
    }

    pub fn as_key(&self) -> String {
        match self {
            Pref::Aichi => String::from("aichi"),
            Pref::Akita => String::from("akita"),
            Pref::Aomori => String::from("aomori"),
            Pref::Chiba => String::from("chiba"),
            Pref::Ehime => String::from("ehime"),
            Pref::Fukui => String::from("fukui"),
            Pref::Fukuoka => String::from("fukuoka"),
            Pref::Fukushima => String::from("fukushima"),
            Pref::Gifu => String::from("gifu"),
            Pref::Gumma => String::from("gumma"),
            Pref::Hiroshima => String::from("hiroshima"),
            Pref::Okayama => String::from("okayama"),
            Pref::Hokkai => String::from("hokkai"),
            Pref::Hyogo => String::from("hyogo"),
            Pref::Ibaraki => String::from("ibaraki"),
            Pref::Ishikawa => String::from("ishikawa"),
            Pref::Iwate => String::from("iwate"),
            Pref::Kagawa => String::from("kagawa"),
            Pref::Kagoshima => String::from("kagoshima"),
            Pref::Kanagawa => String::from("kanagawa"),
            Pref::Kochi => String::from("kochi"),
            Pref::Kumamoto => String::from("kumamoto"),
            Pref::Kyoto => String::from("kyoto"),
            Pref::Mie => String::from("mie"),
            Pref::Miyagi => String::from("miyagi"),
            Pref::Miyazaki => String::from("miyazaki"),
            Pref::Nagano => String::from("nagano"),
            Pref::Nagasaki => String::from("nagasaki"),
            Pref::Nara => String::from("nara"),
            Pref::Niigata => String::from("niigata"),
            Pref::Oita => String::from("oita"),
            Pref::Okinawa => String::from("okinawa"),
            Pref::Osaka => String::from("osaka"),
            Pref::Saga => String::from("saga"),
            Pref::Saitama => String::from("saitama"),
            Pref::Shiga => String::from("shiga"),
            Pref::Shimane => String::from("shimane"),
            Pref::Shizuoka => String::from("shizuoka"),
            Pref::Tochigi => String::from("tochigi"),
            Pref::Tokushima => String::from("tokushima"),
            Pref::Tokyo => String::from("tokyo"),
            Pref::Tottori => String::from("tottori"),
            Pref::Toyama => String::from("toyama"),
            Pref::Wakayama => String::from("wakayama"),
            Pref::Yamagata => String::from("yamagata"),
            Pref::Yamaguchi => String::from("yamaguchi"),
            Pref::Yamanashi => String::from("yamanashi"),
        }
    }

    pub fn as_kanji(&self) -> String {
        match self {
            Self::Aichi => String::from("愛知"),
            Self::Akita => String::from("秋田"),
            Self::Aomori => String::from("青森"),
            Self::Chiba => String::from("千葉"),
            Self::Ehime => String::from("愛媛"),
            Self::Fukui => String::from("福井"),
            Self::Fukuoka => String::from("福岡"),
            Self::Fukushima => String::from("福島"),
            Self::Gifu => String::from("岐阜"),
            Self::Gumma => String::from("群馬"),
            Self::Hiroshima => String::from("広島"),
            Self::Okayama => String::from("岡山"),
            Self::Hokkai => String::from("北海"),
            Self::Hyogo => String::from("兵庫"),
            Self::Ibaraki => String::from("茨城"),
            Self::Ishikawa => String::from("石川"),
            Self::Iwate => String::from("岩手"),
            Self::Kagawa => String::from("香川"),
            Self::Kagoshima => String::from("鹿児島"),
            Self::Kanagawa => String::from("神奈川"),
            Self::Kochi => String::from("高知"),
            Self::Kumamoto => String::from("熊本"),
            Self::Kyoto => String::from("京都"),
            Self::Mie => String::from("三重"),
            Self::Miyagi => String::from("宮城"),
            Self::Miyazaki => String::from("宮崎"),
            Self::Nagano => String::from("長野"),
            Self::Nagasaki => String::from("長崎"),
            Self::Nara => String::from("奈良"),
            Self::Niigata => String::from("新潟"),
            Self::Oita => String::from("大分"),
            Self::Okinawa => String::from("沖縄"),
            Self::Osaka => String::from("大阪"),
            Self::Saga => String::from("佐賀"),
            Self::Saitama => String::from("埼玉"),
            Self::Shiga => String::from("滋賀"),
            Self::Shimane => String::from("島根"),
            Self::Shizuoka => String::from("静岡"),
            Self::Tochigi => String::from("栃木"),
            Self::Tokushima => String::from("徳島"),
            Self::Tokyo => String::from("東京"),
            Self::Tottori => String::from("鳥取"),
            Self::Toyama => String::from("富山"),
            Self::Wakayama => String::from("和歌山"),
            Self::Yamagata => String::from("山形"),
            Self::Yamaguchi => String::from("山口"),
            Self::Yamanashi => String::from("山梨"),
        }
    }

    pub fn suffix(&self) -> String {
        match self {
            Pref::Kyoto | Pref::Osaka => String::from("府"),
            Pref::Tokyo => String::from("都"),
            Pref::Hokkai => String::from("道"),
            _ => String::from("県"),
        }
    }
}
