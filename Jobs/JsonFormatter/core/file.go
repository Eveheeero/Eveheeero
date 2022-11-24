package core

func (f *Formatter) read() string {
	buf := make([]byte, 1024)
	f.reader.Read(buf)
	result := string(buf)
	return result
}
