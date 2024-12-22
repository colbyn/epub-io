Dump contents of the given EPUB file to the specified output directory (`sample/book-output/` in this case):
```
cargo run -- dump --input sample/input-file.epub --output sample/book-output
```
Will create `sample/book-output/` if missing. 

The CLI also supports converting dumped HTML files as plain text as shown in the following:
```
cargo run -- dump --input sample/庄子.epub --output sample/庄子-book-output --html2text --text-width=100
```

# Help

```
➜  cargo run -- --help
A CLI tool for dumping EPUB files and processing content into clean plain text files.

Usage: epub-io <COMMAND>

Commands:
  dump  Dump the contents of the given input EPUB file to the specified output directory
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

```
➜ cargo run -- dump --help
Dump the contents of the given input EPUB file to the specified output directory

Usage: epub-io dump [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
      --input <INPUT>
          The file path of the input EPUB file

      --output <OUTPUT>
          The output directory path.

          Will create the output directory if missing.

      --html2text
          Include plain text versions of the unpacked HTML files using `html2text`

      --text-width <TEXT_WIDTH>
          When the `html2text` flag is on, the text will be wrapped to `width` columns

          [default: 80]

  -h, --help
          Print help (see a summary with '-h')
```