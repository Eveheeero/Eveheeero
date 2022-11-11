use log::debug;

/// translate.google.com에서 사용하는 발송 쿼리문을 생성한다.
pub fn build_google_api_query(text: &String, input_lang: &String, output_lang: &String) -> String {
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
pub async fn send_google_api_query(query: String) -> Result<String, Box<dyn std::error::Error>> {
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

pub async fn translate_one_line(
    text: String,
    input_lang: String,
    output_lang: String,
) -> Result<String, Box<dyn std::error::Error>> {
    // translate.google.com 발송 쿼리문 생성
    let query = build_google_api_query(&text, &input_lang, &output_lang);

    // 번역 후 결과물 (Json형태)
    let text = send_google_api_query(query).await?;

    let content = json::parse(&text).unwrap();

    // 번역이 들어있는 Json의 위치 지정
    let iter = &content[1][0][0][5];

    Ok(iter.members().nth(0).unwrap()[0].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_translate_one_line() {
        let text = "Hello, world!".to_string();
        let input_lang = "en".to_string();
        let output_lang = "ko".to_string();
        let result = translate_one_line(text, input_lang, output_lang)
            .await
            .unwrap();
        dbg!(result);
        assert!(true);
    }
}
