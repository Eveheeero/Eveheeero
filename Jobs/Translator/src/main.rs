use std::io::Write;

use log::debug;

#[tokio::main]
async fn main() {
    // 로거 설정
    // let _ = simplelog::SimpleLogger::init(log::LevelFilter::Debug, simplelog::Config::default());

    print!("Input Text : ");
    std::io::stdout().flush().unwrap();
    let mut text = String::new();
    std::io::stdin().read_line(&mut text).unwrap();

    // 출발 언어와 도착 언어 지정
    let start_lang = "auto";
    print!("Output Language (Ex. ko) : ");
    std::io::stdout().flush().unwrap();
    let mut target_lang = String::new();
    std::io::stdin().read_line(&mut target_lang).unwrap();
    target_lang = target_lang.trim().to_owned();

    let query = {
        // 번역 쿼리문에는 줄바꿈이 \\n으로 들어가있다. 이에 맞추어 보내야한다.
        let text = text.replace("\r\n", "\\\\n").replace("\n", "\\\\n");
        // 쿼리문 설정
        let query = format!(
            // 구글 내부 쿼리문 형태에 따른다
            r#"[[["MkEWBc","[[\"{}\",\"{}\",\"{}\",true],[null]]",null,"generic"]]]"#,
            text, start_lang, target_lang
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

    let content = json::parse(&text).unwrap();
    // 번역이 들어있는 Json의 위치 지정
    let iter = &content[1][0][0][5];
    for i in iter.members().step_by(2) {
        println!("{}", i[0]);
    }
}
