use clap::Parser;
use log::debug;

#[derive(Parser, Debug)]
struct Args {
    /// Input language
    #[arg(short, long = "input", default_value_t = String::from("auto"))]
    input_lang: String,

    /// Output language
    #[arg(short, long = "output", default_value_t = String::from("en"))]
    output_lang: String,

    /// Translator mode, 1 - one line, 2 - file, 3 - Interactive
    #[arg(short = 'm', long = "mode", default_value_t = 1)]
    mode: u8,

    /// Args
    #[arg(short = 'a', long = "args")]
    args: Option<String>,
}

#[tokio::main]
async fn main() {
    // 로거 설정
    // let _ = simplelog::SimpleLogger::init(log::LevelFilter::Debug, simplelog::Config::default());

    translate_one_line("Hello", "auto", "kr").await;

    // 인자 파싱
    let args = Args::parse();

    // 출발 언어와 도착 언어 지정
    let input_lang = args.input_lang;
    let output_lang = args.output_lang;

    match args.mode {
        1 => todo!(),
        2 => todo!(),
        3 => todo!(),
        _ => return,
    }
}

async fn translate_one_line(text: &str, input_lang: &str, output_lang: &str) -> String {
    // translate.google.com 발송 쿼리문 생성
    let query = {
        // 번역 쿼리문에는 줄바꿈이 \\n으로 들어가있다. 이에 맞추어 보내야한다.
        let text = text.replace("\r\n", "\\\\n").replace("\n", "\\\\n");
        // 쿼리문 설정
        let query = format!(
            // 구글 내부 쿼리문 형태에 따른다
            r#"[[["MkEWBc","[[\"{}\",\"{}\",\"{}\",true],[null]]",null,"generic"]]]"#,
            text, input_lang, output_lang
        );
        debug!("query: {}", query);
        query
    };

    // 번역 후 결과물
    let text = {
        // 클라이언트 생성
        let client = reqwest::Client::new();

        // 번역요청 주소 전달
        let builder =
            client.post("https://translate.google.com/_/TranslateWebserverUi/data/batchexecute");
        // content-length 설정
        let builder = builder.header("content-length", "0");
        // 내부 내용 쿼리로 설정 및 전송
        let builder = builder.query(&[("f.req", query)]);
        let response = builder.send().await.unwrap();
        let text = response.text().await.unwrap();
        debug!("response: {}", text);

        // 받아온 반환값 중 불필요한 내용 제거
        let text = text
            .split_at(6)
            .1
            .replace("\\\\", "\\")
            .replace("\\\"", "\"");
        let text = text
            .split_at(21)
            .1
            .split_once(r#"",null,null,null,"generic"],["#)
            .unwrap()
            .0
            .to_owned();
        debug!("text: {}", text);
        text
    };

    let query = {
        // 번역 쿼리문에는 줄바꿈이 \\n으로 들어가있다. 이에 맞추어 보내야한다.
        let text = text.replace("\r\n", "\\\\n").replace("\n", "\\\\n");
        // 쿼리문 설정
        let query = format!(
            // 구글 내부 쿼리문 형태에 따른다
            r#"[[["MkEWBc","[[\"{}\",\"{}\",\"{}\",true],[null]]",null,"generic"]]]"#,
            text, input_lang, output_lang
        );
        debug!("query: {}", query);
        query
    };

    let content = json::parse(&text).unwrap();
    // 번역이 들어있는 Json의 위치 지정
    let iter = &content[1][0][0][5];
    for i in iter.members().step_by(2) {
        println!("{}", i[0]);
    }

    "OK".to_string()
}
