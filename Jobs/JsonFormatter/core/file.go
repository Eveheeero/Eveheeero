package core

func (f *Formatter) read() []byte {
	buf := make([]byte, 1024)
	f.reader.Read(buf)
	return buf
}

func (f *Formatter) write(buf []byte) {
	f.temp_file.Write(buf)
}

func (f *Formatter) save() {

}
