using System.Net;
using System.Net.Http.Json;
using System.Text.Encodings.Web;

namespace Rng.Core.Generator;

class HumanName
{
  private static readonly Lazy<List<string>> familyNameOfKorea = new(() =>
  {
    var data = """
    가
    간
    갈
    감
    강
    견
    경
    계
    고
    곡
    공
    곽
    관
    교
    구
    국
    궁
    궉
    권
    근
    금
    기
    길
    김
    나
    난
    남
    남궁
    낭
    내
    노
    뇌
    다
    단
    담
    당
    대
    도
    독
    독고
    돈
    동
    동방
    두
    등
    등정
    라
    란
    랑
    려
    로
    뢰
    류
    리
    림
    마
    만
    망절
    매
    맹
    명
    모
    목
    묘
    무
    무본
    묵
    문
    미
    민
    박
    반
    방
    배
    백
    번
    범
    변
    보
    복
    봉
    부
    비
    빈
    빙
    사
    사공
    산
    삼
    상
    서
    서문
    석
    선
    선우
    설
    섭
    성
    소
    손
    송
    수
    순
    승
    시
    신
    심
    아
    안
    애
    야
    양
    어
    어금
    엄
    여
    연
    염
    엽
    영
    예
    오
    옥
    온
    옹
    완
    왕
    요
    용
    우
    운
    원
    위
    유
    육
    윤
    은
    음
    이
    인
    임
    자
    장
    전
    점
    정
    제
    제갈
    조
    종
    좌
    주
    증
    지
    진
    차
    창
    채
    천
    초
    총
    최
    추
    탁
    탄
    탕
    태
    판
    팽
    편
    평
    포
    표
    풍
    피
    필
    하
    학
    한
    함
    해
    허
    현
    형
    호
    홍
    화
    황
    황목
    황보
    후
    """;
    return data.Split('\n').ToList();
  });

  public static string FamilyName()
  {
    return familyNameOfKorea.Value[new Random().Next(familyNameOfKorea.Value.Count)];
  }

  public static async Task<String> SearchKoreaNames(string from, string to)
  {
    HttpClient client = new();
    UrlEncoder? encoder = UrlEncoder.Default;
    var query = """
    {0}
      "@MultiCandType": {0} "value": ["DT"], "type": "STRING", "defaultValue": "" {1},
      "@MultiCandStDt": {0}
        "value": ["{2}"],
          "type": "STRING",
          "defaultValue": ""
      {1},
      "@MultiCandEdDt": {0}
        "value": ["{3}"],
          "type": "STRING",
          "defaultValue": ""
      {1},
      "@SidoCd": {0}
        "value": [
              "11",
              "26",
              "27",
              "29",
              "28",
              "30",
              "36",
              "31",
              "41",
              "43",
              "46",
              "44",
              "45",
              "47",
              "48",
              "51",
              "21",
              "50",
              "22",
              "23",
              "24",
              "42",
              "25"
          ],
          "type": "STRING",
          "defaultValue": "[All]",
          "whereClause": "C.SIDO_CD"
      {1},
      "@CggCd": {0}
        "value": ["_EMPTY_VALUE_"],
          "type": "STRING",
          "defaultValue": "[All]",
          "whereClause": "D.CGG_CD"
      {1},
      "@UmdCd": {0}
        "value": ["_EMPTY_VALUE_"],
          "type": "STRING",
          "defaultValue": "[All]",
          "whereClause": "E.UMD_CD"
      {1},
      "@GenderCd": {0}
        "value": ["_EMPTY_VALUE_"],
          "type": "STRING",
          "defaultValue": "[All]",
          "whereClause": "F.GENDER_CD"
      {1}
    {1}
    """;
    var jsonQuery = encoder.Encode(String.Format(query, '{', '}', from, to));
    Console.WriteLine(String.Format(query, '{', '}', from, to));
    var response = await client.PostAsync("https://efamily.scourt.go.kr/ds/report/query.do", new StringContent($"pid=1811&uid=999999&dsid=1261&dstype=DS&mapid=dcea0891-75fa-4cbd-b40f-72986a16abf6&sqlid=1811-1&params={jsonQuery}"));
    var responseJson = await response.Content.ReadAsStringAsync();
    return responseJson;
  }
}