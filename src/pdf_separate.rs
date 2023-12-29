use crate::utils::{run_program, PopplerFile};
use std::future::Future;

pub struct PdfSeparateConfig {
    pub first_page: i32,     // firstPage, -f
    pub last_page: i32,      // lastPage, -l
    pub print_version: bool, // printVersion, -v
    pub print_help: bool,    // printHelp, -h
}

impl Default for PdfSeparateConfig {
    fn default() -> Self {
        Self {
            first_page: 0,
            last_page: 0,
            print_version: false,
            print_help: false,
        }
    }
}

pub fn pdf_separate(
    file: PopplerFile,
    options: PdfSeparateConfig,
) -> impl Future<Output = Result<String, std::io::Error>> {
    let parsed_options = parse_options(&options);

    // return the non-awaited future
    run_program(file, "pdfinfo", parsed_options)
}

fn parse_options(options: &PdfSeparateConfig) -> Vec<String> {
    let mut parsed_options = Vec::new();

    macro_rules! add_option {
        ($condition:expr, $arg:expr) => {
            if $condition {
                parsed_options.push($arg.to_string());
            }
        };
    }
    add_option!(
        options.first_page != 1,
        format!("-f {}", options.first_page)
    );
    add_option!(options.last_page != 0, format!("-l {}", options.last_page));
    add_option!(options.print_version, "-v");
    add_option!(options.print_help, "-h");

    parsed_options
}
