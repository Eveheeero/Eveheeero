package util

import (
	"log"
	"os"
	"strings"
)

func GetAppFolder() string {
	path, err := os.UserConfigDir()
	if err != nil {
		log.Fatalln(err)
		os.Exit(1)
	}
	path = strings.Join([]string{path, "memo"}, "/")
	err = os.MkdirAll(path, 0644)
	if err != nil {
		log.Fatalln(err)
		os.Exit(1)
	}
	return path
}
