# Poppler Utils in Rust

This is a std::process wrapper for the utilities in [Poppler](https://poppler.freedesktop.org/) PDF rendering library.

This library is build to align with Frazer Smith's [node-poppler](https://github.com/Fdawgs/node-poppler). The goal is to provide a Rust alternative to the node wrapper.

**Poppler version: 23.12.0**
The Poppler executables are compiled from source for Mach-O, ELF, and PE formats.
Data encoding for CJK and Cyrillic is included.
NSS signatures & Curl are currently not supported.

### For more complex needs:

It would be best to rebuild the [utils](https://gitlab.freedesktop.org/poppler/poppler/-/tree/master/utils?ref_type=heads) and supporting functionality directly in Rust. A good starting point is the poppler-rs package.

Check out [Amos](fasterthanli.me) to see [documention](https://fasterthanli.me/series/dont-shell-out/) of the journey to build [poppler-rs](https://crates.io/crates/poppler-rs) with the [GIR tooling](https://gtk-rs.org/gir/book/).

## TODO

- [ ] Unix Poppler compile
- [ ] Windows Poppler compile

- [ ] pdfattach
  - [ ] module
  - [ ] tests
  - [ ] docs
- [ ] pdfdetach
  - [ ] module
  - [ ] tests
  - [ ] docs
- [ ] pdffonts
  - [ ] module
  - [ ] tests
  - [ ] docs
- [ ] pdfimages
  - [ ] module
  - [ ] tests
  - [ ] docs
- [ ] pdfinfo
  - [x] module
  - [-] tests
  - [ ] docs
- [ ] pdfseparate
  - [ ] module
  - [ ] tests
  - [ ] docs
- [ ] pdfsig
  - [ ] module
  - [ ] tests
  - [ ] docs
- [ ] pdftocairo
  - [ ] module
  - [ ] tests
  - [ ] docs
- [ ] pdftohtml
  - [x] module
  - [-] tests
  - [-] docs
- [ ] pdftoppm
  - [ ] module
  - [ ] tests
  - [ ] docs
- [ ] pdftops
  - [ ] module
  - [ ] tests
  - [ ] docs
- [ ] pdftotext
  - [x] module
  - [-] tests
  - [ ] docs
- [ ] pdfunite
  - [ ] module
  - [ ] tests
  - [ ] docs

## Usage

### PDF to HTML

```rs
    fn pdf_to_html() {
        let file_path: &str = "test.pdf"; // can be Vec<u8>, &Path, PathBuf, &str, String
        let file: PopplerFile = file_path.as_poppler_path(); // or as_poppler_buffer() for Vec<u8>

        // Uses builder pattern to set options
        let mut config = poppler_utils::PdfToHtmlConfig::default();
        config.zoom = 1.5;

        poppler_utils::pdf_to_html(file, config);
    }
```
