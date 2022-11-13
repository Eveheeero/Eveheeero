use log::debug;

/// 번역 결과물
#[derive(Default, Debug, Clone)]
pub struct TranslateResult {
    pub input_lang: String,
    pub output_lang: String,
    /// 번역 전 문장, 라인별로 구분되어있음
    pub input_text: Vec<String>,
    /// 번역 후 문장, output_text[0]에는 첫번째 라인의 번역 결과물들이 들어있음
    /// output_text[0][0]은 가장 가능성이 높은 결과물
    /// output_text[0][1]은 다음으로 가능성이 높은 결과물
    pub output_text: Vec<Vec<String>>,
    /// 번역 전 문장에 대한 발음
    pub input_tts: Option<Vec<String>>,
    /// 번역 후 문장의 최선의 결과물에 대한 발음
    pub output_tts: Option<Vec<String>>,
}

/// translate.google.com에서 사용하는 발송 쿼리문을 생성한다.
/// Hello\\\\nHello\\nHow Are You
/// Google의 번역기에서 \ 을 입력하면 \\\\로 변환되며, 줄바꿈은 \\n으로 변환된다.
pub fn build_google_api_query(text: &String, input_lang: &String, output_lang: &String) -> String {
    // 번역 쿼리문에는 줄바꿈이 \\n으로 들어가있다. 이에 맞추어 보내야한다.
    let text = text
        .replace("\\", "\\\\")
        .replace("\r\n", "\\n")
        .replace("\n", "\\n")
        .replace("\\", "\\\\");
    // 쿼리문 설정
    let query = format!(
        // 구글 내부 쿼리문 형태에 따른다
        r#"[[["MkEWBc","[[\"{}\",\"{}\",\"{}\",true],[null]]",null,"generic"]]]"#,
        text, input_lang, output_lang
    );
    debug!("Built Query : {}", query);
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
    debug!("Google Response : {}", text);

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
    debug!("Stripped Response : {}", text);
    Ok(text)
}

/// Google 서버에서 받아온 결과 쿼리문을 구조체로 변환한다.
pub fn response_to_result(response: String) -> TranslateResult {
    // 기본 변수 선언
    let mut result = TranslateResult::default();
    let response = json::parse(&response).unwrap();

    // 입출력 언어 저장
    result.input_lang = response[1][4][1].to_string();
    result.output_lang = response[1][4][2].to_string();

    // 입력값 저장
    result.input_text = response[1][4][0]
        .to_string()
        .split('\n')
        .map(|x| x.to_owned())
        .collect();

    // 출력값 저장
    for line in response[1][0][0][5].members().step_by(2) {
        let mut line_result = Vec::new();
        // 최선의 번역결과 저장
        line_result.push(line[0].to_string());
        for side in line[4].members().skip(1) {
            // 그 외의 추측 결과 저장
            line_result.push(side[0].to_string());
        }
        result.output_text.push(line_result);
    }

    // tts 저장
    result.input_tts = if !response[0][0].is_null() {
        Some(
            response[0][0]
                .to_string()
                .split('\n')
                .map(|x| x.to_owned())
                .collect(),
        )
    } else {
        None
    };
    result.output_tts = if !response[1][0][0][1].is_null() {
        Some(
            response[1][0][0][1]
                .to_string()
                .split('\n')
                .map(|x| x.to_owned())
                .collect(),
        )
    } else {
        None
    };

    result
}

/*/////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////// */

pub async fn translate(
    text: Vec<String>,
    input_lang: String,
    output_lang: String,
) -> Result<TranslateResult, Box<dyn std::error::Error>> {
    // 입력값 생성
    let text = text.join("\n");

    // translate.google.com 발송 쿼리문 생성
    let query = build_google_api_query(&text, &input_lang, &output_lang);

    // 번역 후 결과물 (Json형태)
    let response = send_google_api_query(query).await?;

    let result = response_to_result(response);

    Ok(result)
}

pub async fn translate_one_line(
    text: String,
    input_lang: String,
    output_lang: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let text = vec![text];

    let result = translate(text, input_lang, output_lang).await?;

    Ok(result.output_text[0][0].clone())
}

/*/////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////// */

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

    #[tokio::test]
    async fn test_translate_multi_lines() {
        let text = vec!["Hello, world!", "내 이름은 민수야.", "나는 20살이야."]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let input_lang = "auto".to_string();
        let output_lang = "fr".to_string();
        let result = translate(text, input_lang, output_lang).await.unwrap();
        dbg!(result);
        assert!(true);
    }
}