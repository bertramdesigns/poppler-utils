use crate::utils::{run_program, AsPopplerPath, PopplerFile};
use std::future::Future;

pub struct PdfUniteConfig {
    pub print_version: bool, // printVersion, -v
    pub print_help: bool,    // printHelp, -h
}

impl Default for PdfUniteConfig {
    fn default() -> Self {
        Self {
            print_version: false,
            print_help: false,
        }
    }
}

pub fn pdf_unite(
    files: Vec<PopplerFile>,
    options: PdfUniteConfig,
) -> impl Future<Output = Result<String, std::io::Error>> {
    let mut parsed_options = parse_options(&options);

    // for each PopplerFile in files Vec, add the file path to parsed_options
    for file in files {
        match file {
            PopplerFile::Path(path) => {
                if let Ok(path_string) = path.path.into_os_string().into_string() {
                    parsed_options.push(path_string);
                }
            }
            PopplerFile::Buffer(buffer) => {
                if let Ok(buffer_string) = String::from_utf8(buffer.buffer) {
                    parsed_options.push(buffer_string);
                }
            }
        }
    }

    // return the non-awaited future
    // pdfunite takes a blank in first param, so we pass in an empty string.
    // pdfunite [options] <input PDF files> <output PDF file>
    run_program("".as_poppler_path(), "pdfunite", parsed_options)
}

fn parse_options(options: &PdfUniteConfig) -> Vec<String> {
    let mut parsed_options = Vec::new();

    macro_rules! add_option {
        ($condition:expr, $arg:expr) => {
            if $condition {
                parsed_options.push($arg.to_string());
            }
        };
    }
    add_option!(options.print_version, "-v");
    add_option!(options.print_help, "-h");

    parsed_options
}
