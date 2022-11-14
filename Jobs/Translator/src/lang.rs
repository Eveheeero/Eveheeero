#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Default)]
pub enum InputLang {
    #[default]
    Auto,
    Galacian,
    Guaranier,
    Kujarat,
    Greek,
    Dutch,
    Nepal,
    Norwegian,
    Denmark,
    Doger,
    German,
    Divier,
    Laoer,
    Latvian,
    Latin,
    Russian,
    Luganda,
    Romanian,
    Luxembourg,
    Lithuanian,
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
            "gl" => InputLang::Galacian,
            "gn" => InputLang::Guaranier,
            "gu" => InputLang::Kujarat,
            "el" => InputLang::Greek,
            "nl" => InputLang::Dutch,
            "ne" => InputLang::Nepal,
            "no" => InputLang::Norwegian,
            "da" => InputLang::Denmark,
            "doi" => InputLang::Doger,
            "de" => InputLang::German,
            "dv" => InputLang::Divier,
            "lo" => InputLang::Laoer,
            "lv" => InputLang::Latvian,
            "la" => InputLang::Latin,
            "ru" => InputLang::Russian,
            "lg" => InputLang::Luganda,
            "ro" => InputLang::Romanian,
            "lb" => InputLang::Luxembourg,
            "lt" => InputLang::Lithuanian,
            _ => panic!("Invalid input language"),
        }
    }
}

impl ToString for InputLang {
    fn to_string(&self) -> String {
        match self {
            InputLang::Auto => "auto".to_string(),
            InputLang::Galacian => "gl".to_string(),
            InputLang::Guaranier => "gn".to_string(),
            InputLang::Kujarat => "gu".to_string(),
            InputLang::Greek => "el".to_string(),
            InputLang::Dutch => "nl".to_string(),
            InputLang::Nepal => "ne".to_string(),
            InputLang::Norwegian => "no".to_string(),
            InputLang::Denmark => "da".to_string(),
            InputLang::Doger => "doi".to_string(),
            InputLang::German => "de".to_string(),
            InputLang::Divier => "dv".to_string(),
            InputLang::Laoer => "lo".to_string(),
            InputLang::Latvian => "lv".to_string(),
            InputLang::Latin => "la".to_string(),
            InputLang::Russian => "ru".to_string(),
            InputLang::Luganda => "lg".to_string(),
            InputLang::Romanian => "ro".to_string(),
            InputLang::Luxembourg => "lb".to_string(),
            InputLang::Lithuanian => "lt".to_string(),
        }
    }
}

pub enum OutputLang {}
