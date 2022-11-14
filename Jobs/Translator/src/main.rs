use std::io::Write;

use google_translator::*;

use clap::Parser;

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
                        translate_one_line(args, input_lang, output_lang).await?
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
                        translate_one_line(input, input_lang, output_lang).await?
                    ),
                )?;
            }
        },
        // 파일 전체 번역
        2 => {
            // 파일 경로 추출
            let file_path = args.args.unwrap();
            // 데이터를 라인별로 벡터로 생성
            let lines = std::fs::read_to_string(file_path)?
                .split("\n")
                .map(|x| x.trim().to_owned())
                .collect::<Vec<_>>();

            // 5000글자 기준으로 그룹화
            let mut translated = Vec::new();
            let mut count = 0;
            let mut group = Vec::new();
            for line in lines {
                count += line.len();
                if count >= 5000 {
                    translated.push(tokio::spawn(translate(
                        group,
                        input_lang.clone(),
                        output_lang.clone(),
                    )));
                    group = Vec::new();
                    count = line.len();
                }
                group.push(line);
            }
            translated.push(tokio::spawn(translate(group, input_lang, output_lang)));

            for now in translated {
                let result = now.await??;
                for line in result.output_text {
                    print_one_line(&output_file, &format!("{}", line[0]))?;
                }
            }
        }
        // 대화형 번역
        3 => {
            // 사용자가 종료할때까지 번역
            loop {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                // 번역 후 출력
                tokio::spawn(translate_buf(
                    output_file.clone(),
                    input,
                    input_lang.clone(),
                    output_lang.clone(),
                ));
            }
        }
        _ => {}
    }
    Ok(())
}

async fn translate_buf(
    output_file: Option<String>,
    text: String,
    input_lang: String,
    output_lang: String,
) {
    let result = translate_one_line(text, input_lang, output_lang).await;
    match result {
        Ok(result) => {
            print_one_line(&output_file, &format!("{}", result)).unwrap();
        }
        Err(e) => {
            print_one_line(&output_file, &format!("{}", e)).unwrap();
        }
    }
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
