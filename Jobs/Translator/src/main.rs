use std::io::Write;

use clap::Parser;
use log::debug;

#[cfg(windows)]
const EOL: &str = "\r\n";
#[cfg(not(windows))]
const EOL: &str = "\n";

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

    /// Output File
    #[arg(short = 'f', long = "output file")]
    output_file: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 로거 설정
    // let _ = simplelog::SimpleLogger::init(log::LevelFilter::Debug, simplelog::Config::default());

    // 인자 파싱
    let args = Args::parse();
    let output_file = args.output_file;

    // 출발 언어와 도착 언어 지정
    let input_lang = args.input_lang;
    let output_lang = args.output_lang;

    match args.mode {
        // 한 줄 번역
        1 => match args.args {
            // 인자에 있는 값 번역
            Some(args) => {
                // 번역 후 출력
                print_one_line(
                    &output_file,
                    &format!(
                        "{}",
                        translate_one_line(&args, &input_lang, &output_lang).await?
                    ),
                )?;
            }
            // 입력값 번역
            None => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                // 번역 후 출력
                print_one_line(
                    &output_file,
                    &format!(
                        "{}",
                        translate_one_line(&input, &input_lang, &output_lang).await?
                    ),
                )?;
            }
        },
        // 파일 전체 번역
        2 => todo!(),
        // 대화형 번역
        3 => {
            // 사용자가 종료할때까지 번역
            loop {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                // 번역 후 출력
                print_one_line(
                    &output_file,
                    &format!(
                        "{}",
                        translate_one_line(&input, &input_lang, &output_lang).await?
                    ),
                )?;
            }
        }
        _ => {}
    }
    Ok(())
}

/// translate.google.com에서 사용하는 발송 쿼리문을 생성한다.
fn build_google_api_query(text: &String, input_lang: &String, output_lang: &String) -> String {
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
}

/// translate.google.com의 api 서버에 요청을 보낸 후 쓸모없는 값을 제거한다.
/// 출력값은 Json 형태이다.
/// object[1][0][0][5] 형태로 번역 결과에 접근할 수 있으며, 해당 부분의 배열에 2단계씩 건너뛰어 번역 결과가 들어있다.
async fn send_google_api_query(query: String) -> Result<String, Box<dyn std::error::Error>> {
    // 클라이언트 생성
    let client = reqwest::Client::new();

    // 번역요청 주소 전달
    let builder =
        client.post("https://translate.google.com/_/TranslateWebserverUi/data/batchexecute");
    // content-length 설정
    let builder = builder.header("content-length", "0");
    // 내부 내용 쿼리로 설정 및 전송
    let builder = builder.query(&[("f.req", query)]);
    let response = builder.send().await?;
    let text = response.text().await?;
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
    Ok(text)
}

async fn translate_one_line(
    text: &String,
    input_lang: &String,
    output_lang: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    // translate.google.com 발송 쿼리문 생성
    let query = build_google_api_query(text, input_lang, output_lang);

    // 번역 후 결과물 (Json형태)
    let text = send_google_api_query(query).await?;

    let content = json::parse(&text).unwrap();

    // 번역이 들어있는 Json의 위치 지정
    let iter = &content[1][0][0][5];

    Ok(iter.members().nth(0).unwrap()[0].to_string())
}

/// 한 라인을 출력한다.
fn print_one_line(
    output_file: &Option<String>,
    line: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    match output_file {
        Some(output_file) => {
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(output_file)?;
            file.write_all(line.as_bytes())?;
            file.write_all(EOL.as_bytes())?;
        }
        None => println!("{}", line),
    };
    Ok(())
}
