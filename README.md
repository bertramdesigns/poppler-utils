# Poppler Utils in Rust

This is a std::process wrapper for the utilities in [Poppler](https://poppler.freedesktop.org/) PDF rendering library.

This library uses the same general concept as [node-poppler](https://github.com/Fdawgs/node-poppler), writing an interface and including required .dll files.

Poppler version: 23.12.0

### For more complex needs:

It would be best to rebuild the [utils](https://gitlab.freedesktop.org/poppler/poppler/-/tree/master/utils?ref_type=heads) and supporting functionality directly in Rust. A good starting point is the poppler-rs package.

Check out [Amos](fasterthanli.me) to see [documention](https://fasterthanli.me/series/dont-shell-out/) of the journey to build [poppler-rs](https://crates.io/crates/poppler-rs) with the [GIR tooling](https://gtk-rs.org/gir/book/).

## TODO

- [ ] pdfattach
- [ ] pdfdetach
- [ ] pdffonts
- [ ] pdfimages
- [ ] pdfinfo
- [ ] pdfseparate
- [ ] pdfsig
- [ ] pdftocairo
- [ ] pdftohtml
- [ ] pdftoppm
- [ ] pdftops
- [ ] pdftotext
- [ ] pdfunite

## Usage

### PDF to HTML

```rs
let mut config = PdfToHtmlConfig::default();
config.scale = 2;
config.first_page = 1;
config.last_page =  5,
let result = PdfToHtml.convert(filename, config);
```
