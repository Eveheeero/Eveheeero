use std::io::{BufRead, Write};

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
                        translate_one_line(args, input_lang.clone(), output_lang.clone()).await?
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
                        translate_one_line(input, input_lang.clone(), output_lang.clone()).await?
                    ),
                )?;
            }
        },
        // 파일 전체 번역
        2 => {
            let file_path = args.args.unwrap();
            let file = std::fs::File::open(file_path)?;
            let mut buf = std::io::BufReader::new(file);
            let mut lines = Vec::new();
            loop {
                let mut line = String::new();
                let len = buf.read_line(&mut line)?;
                if len == 0 {
                    break;
                }
                lines.push(translate_one_line(
                    line,
                    input_lang.clone(),
                    output_lang.clone(),
                ));
            }
            for line in lines {
                print_one_line(&output_file, &format!("{}", line.await?))?;
            }
        }
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
                        translate_one_line(input, input_lang.clone(), output_lang.clone()).await?
                    ),
                )?;
            }
        }
        _ => {}
    }
    Ok(())
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
