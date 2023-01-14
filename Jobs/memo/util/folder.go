package util

import (
	"log"
	"os"
	"strings"
)

// 실행권한이 없으면 폴더생성이 안되는듯하다.

// 앱 폴더
func folder_base() string {

	// 앱 폴더 가져오기
	path, err := os.UserConfigDir()
	if err != nil {
		log.Fatalln(err)
		os.Exit(1)
	}

	// 앱 데이터 경로 생성
	path = strings.Join([]string{path, "memo"}, "/")

	// 앱 데이터 폴더 생성
	err = os.MkdirAll(path, 0744)
	if err != nil {
		log.Fatalln(err)
		os.Exit(1)
	}

	return path
}

// 날짜 폴더
func Folder_date() string {
	// 베이스 폴더 가져오기
	path := folder_base()

	// 날짜 폴더 경로 생성
	path = strings.Join([]string{path, "date"}, "/")
	// 날짜 폴더 생성
	err := os.MkdirAll(path, 0744)
	if err != nil {
		log.Fatalln(err)
		os.Exit(1)
	}

	return path
}

// 키워드 폴더
func Folder_keyword() string {
	// 베이스 폴더 가져오기
	path := folder_base()

	// 날짜 폴더 경로 생성
	path = strings.Join([]string{path, "keyword"}, "/")
	// 날짜 폴더 생성
	err := os.MkdirAll(path, 0744)
	if err != nil {
		log.Fatalln(err)
		os.Exit(1)
	}

	return path
}
