#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Default)]
pub enum InputLang {
    #[default]
    Auto,
    Galego,   // 갈리시아어 - gl
    Guarani,  // 과라니어 - gn
    Gujarati, // 구자라트어 - gu
    Greek,    // 그리스어 - el
    // TODO
    Dutch,         // 네덜란드어 - nl
    Nepali,        // 네팔어 - ne
    Norwegian,     // 노르웨이어 - no
    Danish,        // 덴마크어 - da
    Dogri,         // 도그리어 - doi
    German,        // 독일어 - de
    Dhivehi,       // 디베히어 - dv
    Lao,           // 라오어 - lo
    Latvian,       // 라트비아어 - lv
    Latin,         // 라틴어 - la
    Russian,       // 러시아어 - ru
    Luganda,       // 루간다어 - lg
    Romanian,      // 루마니아어 - ro
    Luxembourgish, // 룩셈부르크어 - lb
    Lithuanian,    // 리투아니아어 - lt
    Lingala,       // 링갈라어 - ln
    Marathi,       // 마라티어 - mr
    Maori,         // 마오리어 - mi
    Maithili,      // 마이틸어 - mai
    Macedonian,    // 마케도니아어 - mk
    Malagasy,      // 말라가시어 - mg
    Malayalam,     // 말라얄람어 - ml
    Malay,         // 말레이어 - ms
    Meithei,       // 메이테이어(마니푸르어) - mni-Mtei
    Malti,         // 몰타어 - mt
    Mongolian,     // 몽골어 - mn
                   // 몽어 - hmn
                   // 미얀마어(버마어) - my
                   // 미조어 - lus
                   // 바스크어 - eu
                   // 밤바라어 - bm
                   // 베트남어 - vi
                   // 벨라루스어 - be
                   // 벵골어 - bn
                   // 보스니아어 - bs
                   // 보즈푸리어 - bho
                   // 복소토어 - nso
                   // 불가리아어 - bg
                   // 사모아어 - sm
                   // 산스크리트 - sa
                   // 세르비아어 - sr
                   // 세부아노어 - ceb
                   // 세소토어 - st
                   // 소말리아어 - so
                   // 쇼나어 - sn
                   // 순다어 - su
                   // 스와힐리어 - sw
                   // 스웨덴어 - sv
                   // 스코틀랜드 게일어 - gd
                   // 스페인어 - es
                   // 슬로바키아어 - sk
                   // 슬로베니아어 - sl
                   // 신디어 - sd
                   // 싱할라어 - si
                   // 아랍어 - ar
                   // 아르메니아어 - hy
                   // 아삼어 - as
                   // 아이마라어 - ay
                   // 아이슬란드어 - is
                   // 아이티 크리올어 - ht
                   // 아일랜드어 - ga
                   // 아제르바이잔어 - az
                   // 아프리칸스어 - af
                   // 알바니아어 - sq
                   // 암하라어 - am
                   // 에스토니아어 - et
                   // 에스페란토어 - eo
                   // 에웨어 - ee
                   // 영어 - en
                   // 오로모어 - om
                   // 오리야어 - or
                   // 요루바어 - yo
                   // 우르두어 - ur
                   // 우즈베크어 - uz
                   // 우크라이나어 - uk
                   // 웨일즈어 - cy
                   // 위구르어 - ug
                   // 이그보어 - ig
                   // 이디시어 - yi
                   // 이탈리아어 - it
                   // 인도네시아어 - id
                   // 일로카노어 - ilo
                   // 일본어 - ja
                   // 자바어 - jw
                   // 조지아어 - ka
                   // 줄루어 - zu
                   // 중국어(간체) - zh-CN
                   // 중국어(번체) - zh-TW
                   // 체와어 - ny
                   // 체코어 - cs
                   // 총가어 - ts
                   // 카자흐어 - kk
                   // 카탈로니아어 - ca
                   // 칸나다어 kn
                   // 케추아어 - qu
                   // 코르시카어 - co
                   // 코사어 - xh
                   // 콘칸어 - gom
                   // 쿠르드어(소라니) - ckb
                   // 쿠트드어(쿠르만지) - ku
                   // 크로아티아어 - hr
                   // 크리오어 - kri
                   // 크메르어 - km
                   // 키냐르완다어 - rw
                   // 키르기스어 - ky
                   // 타밀어 - ta
                   // 타지크어 - tg
                   // 타타르어 - tt
                   // 태국어 - th
                   // 터키어 - tr
                   // 텔루구어 - te
                   // 투르크멘어 - tk
                   // 트위어 - ak
                   // 티그리냐어 - ti
                   // 파슈토어 - ps
                   // 펀자브어 - pa
                   // 페르시아어 - fa
                   // 포르투갈어 - pt
                   // 폴란드어 - pl
                   // 프랑스어 - fr
                   // 프리지아어 - fy
                   // 핀란드어 - fi
                   // 필리핀어 - tl
                   // 하와이어 - haw
                   // 하우사어 - ha
                   // 한국어 - ko
                   // 헝가리어 - hu
                   // 히브리어 - iw
                   // 힌디어 - hi
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
        };
        lang.to_owned()
    }
}

pub enum OutputLang {}
