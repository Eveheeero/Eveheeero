package core

import "os"

type Formatter struct {
	url    string
	reader *os.File
}

func NewFormatter(url string) (*Formatter, error) {
	file, error := os.Open(url)
	if error != nil {
		return nil, error
	}

	return &Formatter{url: url, reader: file}, nil
}

func (f *Formatter) Run() {
	readed := f.read()
	println(readed)
}
