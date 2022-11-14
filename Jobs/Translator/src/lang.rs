#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Default)]
pub enum InputLang {
    #[default]
    Auto,
    Galego,   // 갈리시아어 - gl
    Guarani,  // 과라니어 - gn
    Gujarati, // 구자라트어 - gu
    Greek,    // 그리스어 - el
    // TODO
    Dutch,              // 네덜란드어 - nl
    Nepali,             // 네팔어 - ne
    Norwegian,          // 노르웨이어 - no
    Danish,             // 덴마크어 - da
    Dogri,              // 도그리어 - doi
    German,             // 독일어 - de
    Dhivehi,            // 디베히어 - dv
    Lao,                // 라오어 - lo
    Latvian,            // 라트비아어 - lv
    Latin,              // 라틴어 - la
    Russian,            // 러시아어 - ru
    Luganda,            // 루간다어 - lg
    Romanian,           // 루마니아어 - ro
    Luxembourgish,      // 룩셈부르크어 - lb
    Lithuanian,         // 리투아니아어 - lt
    Lingala,            // 링갈라어 - ln
    Marathi,            // 마라티어 - mr
    Maori,              // 마오리어 - mi
    Maithili,           // 마이틸어 - mai
    Macedonian,         // 마케도니아어 - mk
    Malagasy,           // 말라가시어 - mg
    Malayalam,          // 말라얄람어 - ml
    Malay,              // 말레이어 - ms
    Meithei,            // 메이테이어(마니푸르어) - mni-Mtei
    Malti,              // 몰타어 - mt
    Mongolian,          // 몽골어 - mn
    Hmong,              // 몽어 - hmn
    Burmese,            // 미얀마어(버마어) - my
    Mizo,               // 미조어 - lus
    Basque,             // 바스크어 - eu
    Bambara,            // 밤바라어 - bm
    Vietnamese,         // 베트남어 - vi
    Belarusian,         // 벨라루스어 - be
    Bengali,            // 벵골어 - bn
    Bosnian,            // 보스니아어 - bs
    Bhojpuri,           // 보즈푸리어 - bho
    NSotho,             // 북소토어 - nso
    Bulgarian,          // 불가리아어 - bg
    Samoan,             // 사모아어 - sm
    Sanskrit,           // 산스크리트 - sa
    Serbian,            // 세르비아어 - sr
    Cebuano,            // 세부아노어 - ceb
    Sotho,              // 세소토어 - st
    Somali,             // 소말리아어 - so
    Shona,              // 쇼나어 - sn
    Sundanese,          // 순다어 - su
    Swahili,            // 스와힐리어 - sw
    Swedish,            // 스웨덴어 - sv
    ScottishGaelic,     // 스코틀랜드 게일어 - gd
    Spanish,            // 스페인어 - es
    Slovak,             // 슬로바키아어 - sk
    Slovene,            // 슬로베니아어 - sl
    Sindhi,             // 신디어 - sd
    Sinhala,            // 싱할라어 - si
    Arabic,             // 아랍어 - ar
    Armenian,           // 아르메니아어 - hy
    Assamese,           // 아삼어 - as
    Aymara,             // 아이마라어 - ay
    Icelandic,          // 아이슬란드어 - is
    HaitianCreole,      // 아이티 크리올어 - ht
    Irish,              // 아일랜드어 - ga
    Azerbaijani,        // 아제르바이잔어 - az
    Afrikaans,          // 아프리칸스어 - af
    Albanian,           // 알바니아어 - sq
    Amharic,            // 암하라어 - am
    Estonian,           // 에스토니아어 - et
    Esperanto,          // 에스페란토어 - eo
    Ewe,                // 에웨어 - ee
    English,            // 영어 - en
    Oromo,              // 오로모어 - om
    Odia,               // 오리야어 - or
    Yoruba,             // 요루바어 - yo
    Urdu,               // 우르두어 - ur
    Uzbek,              // 우즈베크어 - uz
    Ukrainian,          // 우크라이나어 - uk
    Welsh,              // 웨일즈어 - cy
    Uyghur,             // 위구르어 - ug
    Igbo,               // 이그보어 - ig
    Yiddish,            // 이디시어 - yi
    Italian,            // 이탈리아어 - it
    Indonesian,         // 인도네시아어 - id
    Ilocano,            // 일로카노어 - ilo
    Japanese,           // 일본어 - ja
    Javanese,           // 자바어 - jw
    Georgian,           // 조지아어 - ka
    Zulu,               // 줄루어 - zu
    SimplifiedChinese,  // 중국어(간체) - zh-CN
    TraditionalChinese, // 중국어(번체) - zh-TW
    Chewa,              // 체와어 - ny
    Czech,              // 체코어 - cs
    Tsonga,             // 총가어 - ts
    Kazakh,             // 카자흐어 - kk
    Catalan,            // 카탈로니아어 - ca
    Kannada,            // 칸나다어 kn
    Quechuan,           // 케추아어 - qu
    Corsican,           // 코르시카어 - co
    Xhosa,              // 코사어 - xh
    Konkani,            // 콘칸어 - gom
    Sorani,             // 쿠르드어(소라니) - ckb
    Kurmanji,           // 쿠트드어(쿠르만지) - ku
    Croatian,           // 크로아티아어 - hr
    Krio,               // 크리오어 - kri
    Khmer,              // 크메르어 - km
    Kinyarwanda,        // 키냐르완다어 - rw
    Kyrgyz,             // 키르기스어 - ky
    Tamil,              // 타밀어 - ta
    Tajik,              // 타지크어 - tg
    Tatar,              // 타타르어 - tt
    Thai,               // 태국어 - th
    Turkish,            // 터키어 - tr
    Telugu,             // 텔루구어 - te
    Turkmen,            // 투르크멘어 - tk
    Akan,               // 트위어 - ak
    Tigrinya,           // 티그리냐어 - ti
    Pashto,             // 파슈토어 - ps
    Punjabi,            // 펀자브어 - pa
    Persian,            // 페르시아어 - fa
    Portuguese,         // 포르투갈어 - pt
    Polish,             // 폴란드어 - pl
    French,             // 프랑스어 - fr
    Frisian,            // 프리지아어 - fy
    Finnish,            // 핀란드어 - fi
    Filipino,           // 필리핀어 - tl
    Hawaiian,           // 하와이어 - haw
    Hausa,              // 하우사어 - ha
    Korean,             // 한국어 - ko
    Hungarian,          // 헝가리어 - hu
    Hebrew,             // 히브리어 - iw
    Hindi,              // 힌디어 - hi
}

unsafe impl Send for InputLang {}

impl Into<InputLang> for String {
    fn into(self) -> InputLang {
        self.as_str().into()
    }
}

impl Into<InputLang> for &String {
    fn into(self) -> InputLang {
        self.as_str().into()
    }
}

impl Into<InputLang> for &str {
    fn into(self) -> InputLang {
        let data = self.to_lowercase();
        match data.as_str() {
            "auto" => InputLang::Auto,

            // 갈리시아어
            "gl" | "glg" => InputLang::Galego,
            "galego" => InputLang::Galego,
            "galicia" => InputLang::Galego,
            "galician" => InputLang::Galego,
            "gallego" => InputLang::Galego,

            // 과라니어
            "gn" | "grn" | "nhd" | "gui" | "gun" | "gug" | "gnw" => InputLang::Guarani,
            "guaraní" | "guarani" => InputLang::Guarani,
            "avañe'ẽ" => InputLang::Guarani,

            // 구자라트어
            "gu" | "guj" => InputLang::Gujarati,
            "gujarati" => InputLang::Gujarati,
            "gujarat" => InputLang::Gujarati,
            "ગુજરાતી" => InputLang::Gujarati,
            "ગુજરાત" => InputLang::Gujarati,
            "gujarātī" => InputLang::Gujarati,

            // 그리스어
            "el" | "gre" | "ell" | "grc" | "cpg" | "gmy" | "pnt" | "tsd" | "yej" => {
                InputLang::Greek
            }
            "eλληνικά" => InputLang::Greek,
            "elliniká" => InputLang::Greek,
            "Ἑλληνική" => InputLang::Greek,
            "eλληνική" => InputLang::Greek,
            "ελληνικά" => InputLang::Greek,
            "hellēnikḗ" => InputLang::Greek,
            "greece" => InputLang::Greek,
            "hellenic" => InputLang::Greek,

            // 그 외
            _ => InputLang::default(),
        }
    }
}

impl ToString for InputLang {
    fn to_string(&self) -> String {
        let lang = match self {
            InputLang::Auto => "auto",
            InputLang::Galego => "gl",
            InputLang::Guarani => "gn",
            InputLang::Gujarati => "gu",
            InputLang::Greek => "el",
            InputLang::Dutch => "nl",
            InputLang::Nepali => "ne",
            InputLang::Norwegian => "no",
            InputLang::Danish => "da",
            InputLang::Dogri => "doi",
            InputLang::German => "de",
            InputLang::Dhivehi => "dv",
            InputLang::Lao => "lo",
            InputLang::Latvian => "lv",
            InputLang::Latin => "la",
            InputLang::Russian => "ru",
            InputLang::Luganda => "lg",
            InputLang::Romanian => "ro",
            InputLang::Luxembourgish => "lb",
            InputLang::Lithuanian => "lt",
            InputLang::Lingala => "ln",
            InputLang::Marathi => "mr",
            InputLang::Maori => "mi",
            InputLang::Maithili => "mai",
            InputLang::Macedonian => "mk",
            InputLang::Malagasy => "mg",
            InputLang::Malayalam => "ml",
            InputLang::Malay => "ms",
            InputLang::Meithei => "mni-Mtei",
            InputLang::Malti => "mt",
            InputLang::Mongolian => "mn",
            InputLang::Hmong => "hmn",
            InputLang::Burmese => "my",
            InputLang::Mizo => "lus",
            InputLang::Basque => "eu",
            InputLang::Bambara => "bm",
            InputLang::Vietnamese => "vi",
            InputLang::Belarusian => "be",
            InputLang::Bengali => "bn",
            InputLang::Bosnian => "bs",
            InputLang::Bhojpuri => "bho",
            InputLang::NSotho => "nso",
            InputLang::Bulgarian => "bg",
            InputLang::Samoan => "sm",
            InputLang::Sanskrit => "sa",
            InputLang::Serbian => "sr",
            InputLang::Cebuano => "ceb",
            InputLang::Sotho => "st",
            InputLang::Somali => "so",
            InputLang::Shona => "sn",
            InputLang::Sundanese => "su",
            InputLang::Swahili => "sw",
            InputLang::Swedish => "sv",
            InputLang::ScottishGaelic => "gd",
            InputLang::Spanish => "es",
            InputLang::Slovak => "sk",
            InputLang::Slovene => "sl",
            InputLang::Sindhi => "sd",
            InputLang::Sinhala => "si",
            InputLang::Arabic => "ar",
            InputLang::Armenian => "hy",
            InputLang::Assamese => "as",
            InputLang::Aymara => "ay",
            InputLang::Icelandic => "is",
            InputLang::HaitianCreole => "ht",
            InputLang::Irish => "ga",
            InputLang::Azerbaijani => "az",
            InputLang::Afrikaans => "af",
            InputLang::Albanian => "sq",
            InputLang::Amharic => "am",
            InputLang::Estonian => "et",
            InputLang::Esperanto => "eo",
            InputLang::Ewe => "ee",
            InputLang::English => "en",
            InputLang::Oromo => "om",
            InputLang::Odia => "or",
            InputLang::Yoruba => "yo",
            InputLang::Urdu => "ur",
            InputLang::Uzbek => "uz",
            InputLang::Ukrainian => "uk",
            InputLang::Welsh => "cy",
            InputLang::Uyghur => "ug",
            InputLang::Igbo => "ig",
            InputLang::Yiddish => "yi",
            InputLang::Italian => "it",
            InputLang::Indonesian => "id",
            InputLang::Ilocano => "ilo",
            InputLang::Japanese => "ja",
            InputLang::Javanese => "jw",
            InputLang::Georgian => "ka",
            InputLang::Zulu => "zu",
            InputLang::SimplifiedChinese => "zh-CN",
            InputLang::TraditionalChinese => "zh-TW",
            InputLang::Chewa => "ny",
            InputLang::Czech => "cs",
            InputLang::Tsonga => "ts",
            InputLang::Kazakh => "kk",
            InputLang::Catalan => "ca",
            InputLang::Kannada => "kn",
            InputLang::Quechuan => "qu",
            InputLang::Corsican => "co",
            InputLang::Xhosa => "xh",
            InputLang::Konkani => "gom",
            InputLang::Sorani => "ckb",
            InputLang::Kurmanji => "ku",
            InputLang::Croatian => "hr",
            InputLang::Krio => "kri",
            InputLang::Khmer => "km",
            InputLang::Kinyarwanda => "rw",
            InputLang::Kyrgyz => "ky",
            InputLang::Tamil => "ta",
            InputLang::Tajik => "tg",
            InputLang::Tatar => "tt",
            InputLang::Thai => "th",
            InputLang::Turkish => "tr",
            InputLang::Telugu => "te",
            InputLang::Turkmen => "tk",
            InputLang::Akan => "ak",
            InputLang::Tigrinya => "ti",
            InputLang::Pashto => "ps",
            InputLang::Punjabi => "pa",
            InputLang::Persian => "fa",
            InputLang::Portuguese => "pt",
            InputLang::Polish => "pl",
            InputLang::French => "fr",
            InputLang::Frisian => "fy",
            InputLang::Finnish => "fi",
            InputLang::Filipino => "tl",
            InputLang::Hawaiian => "haw",
            InputLang::Hausa => "ha",
            InputLang::Korean => "ko",
            InputLang::Hungarian => "hu",
            InputLang::Hebrew => "iw",
            InputLang::Hindi => "hi",
        };
        lang.to_owned()
    }
}

pub enum OutputLang {}
