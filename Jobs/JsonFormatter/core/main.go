package core

import "os"

type Formatter struct {
	url       string
	reader    *os.File
	temp_file *os.File
}

func NewFormatter(url string) (*Formatter, error) {
	file, error := os.Open(url)
	if error != nil {
		return nil, error
	}

	temp_file, error := os.CreateTemp("*", "*")
	if error != nil {
		return nil, error
	}

	return &Formatter{url: url, reader: file, temp_file: temp_file}, nil
}

func (f *Formatter) Run() {
	readed := f.read()

	var in_string bool
	level := 0
	for i, b := range readed {
		switch b {
		case '{':
			if !in_string {
				level += 1
			}
		case '}':
		}
	}
}
