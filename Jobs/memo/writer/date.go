package writer

import (
	"fmt"
	"memo/util"
	"os"
	"strings"
	"time"
)

func DefaultDateWriter() {
	path := util.GetAppFolder()
	path = strings.Join([]string{path, "date"}, "/")
	os.Mkdir(path, 0644)
	println(path)

	now := time.Now().Local()
	year, month, day := now.Date()
	now_file := fmt.Sprintf("%d-%d-%d", year, month, day)
	path = strings.Join([]string{path, now_file}, "/")

	os.Create(path)
	file, err := os.OpenFile(path, os.O_APPEND, 0644)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	println(path)
}
