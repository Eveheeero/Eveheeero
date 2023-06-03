# Simple Translator

**This Program use Google's undocumented api, service may not be available at some day...**

If translation doesn't work well, You can change the API of the source code according to the API format transmitted from inside Google translator.

## Install

```sh
cargo install google_translator
```

## Usage

```text
# translate --help
Usage: translate [OPTIONS]

Options:
  -i, --input <INPUT_LANG>         Input language [default: auto]
  -o, --output <OUTPUT_LANG>       Output language [default: en]
  -m, --mode <MODE>                Translator mode, 1 - one line, 2 - file, 3 - Interactive [default: 1]
  -a, --args <ARGS>                Args
  -f, --output file <OUTPUT_FILE>  Output File
  -h, --help                       Print help information
```

or...

```rust
#[tokio::test]
async fn test_translate_multi_lines() {
    let text = vec!["Hello, world!", "내 이름은 민수야.", "나는 20살이야."]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    let input_lang = "auto";
    let output_lang = "fr";
    let result = translate(&text, input_lang, output_lang).await.unwrap();
    dbg!(result);
    assert!(true);
}
```

### For example

- translate \<Enter> is Console input, One line translation, Console output.
- translate -i en -o ko -m 3 \<Enter> is Interactive console input, Multi line translation, Console output.
- translate -m 2 -a foo.txt -f bar.txt \<Enter> is File input, Multi line translation, Console output.

## Version

- ... - Enable error handling for many input values (only about to executable program, inputs are divided by 4000 char)
- 0.2.2 - Improved parallel processing for executable files, corrected errors for quotation marks, and enumerated target language
- 0.2.1 - Library Available, also structures.
- 0.2.0 - Three mod Translate
- 0.1.0 - Publish Cargo and Simple Translate

## End

The project is a toy project developed based on content that was used for a while while working, and may or may not be improved, error corrected.

Because it uses Google's undocumented API, please be careful when using it.
