# Simple Translator

**This Program use Google's undocumented api, service may not be available at some day...**

## Install

```sh
cargo install google_translator
```

## Usage

```text
# google_translator --help
Usage: google_translator [OPTIONS]

Options:
  -i, --input <INPUT_LANG>         Input language [default: auto]
  -o, --output <OUTPUT_LANG>       Output language [default: en]
  -m, --mode <MODE>                Translator mode, 1 - one line, 2 - file, 3 - Interactive [default: 1]
  -a, --args <ARGS>                Args
  -f, --output file <OUTPUT_FILE>  Output File
  -h, --help                       Print help information
```

### For example

- google_translator \<Enter> is Console input, One line translation, Console output.
- google_translator -i en -o ko -m 3 \<Enter> is Interactive console input, Multi line translation, Console output.
- google_translator -m 2 -a foo.txt -f bar.txt \<Enter> is File input, Multi line translation, Console output.

## Hacking

If translation doesn't work well, You can change the API of the source code according to the API format transmitted from inside Google translator.

In source, build_google_api_query function is prepared for multiple lines of input using \\r\\n, and send_google_api_query function is prepared for multiple lines of output.

If you want to get multiple lines of output, see the comment of send_google_api_query function, you can get values for similar results.

## End

The project is a toy project developed based on content that was used for a while while working, and may or may not be improved, libraryized, and error corrected.

Because it uses Google's undocumented API, please be careful when using it.
