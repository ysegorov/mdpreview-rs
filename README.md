# mdpreview

Simple utility to preview markdown file in browser. Written in [Rust][rust].
Uses [github-markdown-css][github-markdown-css] for styling.


## Usage

```sh
$ mdpreview --help
mdpreview 0.1.0
Simple tool to preview markdown file in browser

USAGE:
    mdpreview [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --port <port>    Port number to bind preview server to [default: 4000]
```

Command to start web server on default port (4000):

```sh
$ mdpreview
Listening on http://127.0.0.1:4000
```

Command to start web server on a custom port, say 3443:

```sh
$ mdpreview -p 3443
Listening on http://127.0.0.1:3443
```

Use your web browser to navigate to
[http://127.0.0.1:4000](http://127.0.0.1:4000) to see the list of
available markdown files to preview, click markdown file link and get it
rendered.


## How to build

You must have [Rust installed][rust-install].

Clone this repository and build:

```sh
$ git clone https://github.com/ysegorov/mdpreview-rs
$ cd mdpreview-rs
$ cargo build --release
```

Binary named `mdpreview` will be available in `target/release/` folder.


## License

[Unlicense][unlicense].


[unlicense]: http://unlicense.org
[rust]: https://www.rust-lang.org
[rust-install]: https://www.rust-lang.org/tools/install
[github-markdown-css]: https://github.com/sindresorhus/github-markdown-css
