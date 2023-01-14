package writer

import (
	"fmt"
	"log"
	"memo/util"
	"os"
	"strings"
	"time"
)

func generate_file(time time.Time) *os.File {
	year, month, day := time.Date()
	filename := fmt.Sprintf("%d-%d-%d", year, month, day)

	base_folder := util.Folder_date()
	path := strings.Join([]string{base_folder, filename}, "/")

	if _, err := os.Stat(path); os.IsNotExist(err) {
		os.Create(path)
	}

	file, err := os.OpenFile(path, os.O_APPEND|os.O_WRONLY, 0600)
	if err != nil {
		log.Fatalln(err)
		os.Exit(1)
	}

	return file
}

func DefaultDateWriter() {
	now := time.Now().Local()
	file := generate_file(now)
	defer file.Close()

	_, err := file.WriteString("Hello, World!\n")
	if err != nil {
		log.Fatalln(err)
		os.Exit(1)
	}
}
