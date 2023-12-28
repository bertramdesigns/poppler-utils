use crate::utils::{run_program, PopplerFile};
use std::future::Future;

pub struct PdfInfoConfig<'a> {
    pub first_page: i32,            // firstPage, -f
    pub last_page: i32,             // lastPage, -l
    pub print_boxes: bool,          // printBoxes, -box
    pub print_metadata: bool,       // printMetadata, -meta
    pub print_custom: bool,         // printCustom, -custom
    pub print_js: bool,             // printJS, -js
    pub print_structure: bool,      // printStructure, -struct
    pub print_structure_text: bool, // printStructureText, -struct-text
    pub iso_dates: bool,            // isoDates, -isodates
    pub raw_dates: bool,            // rawDates, -rawdates
    pub print_dests: bool,          // printDests, -dests
    pub print_urls: bool,           // printUrls, -url
    pub output_encoding: &'a str,   // textEncName, -enc
    pub print_enc: bool,            // printEnc, -listenc
    pub owner_password: &'a str,    // ownerPassword, -opw
    pub user_password: &'a str,     // userPassword, -upw
    pub print_version: bool,        // printVersion, -v
    pub print_help: bool,           // printHelp, -h
}

impl<'a> Default for PdfInfoConfig<'a> {
    fn default() -> Self {
        Self {
            first_page: 1,
            last_page: 0,
            print_boxes: false,
            print_metadata: false,
            print_custom: false,
            print_js: false,
            print_structure: false,
            print_structure_text: false,
            iso_dates: false,
            raw_dates: false,
            print_dests: false,
            print_urls: false,
            output_encoding: "",
            print_enc: false,
            owner_password: "",
            user_password: "",
            print_version: false,
            print_help: false,
        }
    }
}

pub fn pdf_info(
    file: PopplerFile,
    options: PdfInfoConfig<'static>,
) -> impl Future<Output = Result<String, std::io::Error>> {
    let parsed_options = parse_options(&options);

    // return the non-awaited future
    run_program(file, "pdfinfo", parsed_options)
}

fn parse_options(options: &PdfInfoConfig) -> Vec<String> {
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
    add_option!(options.print_boxes, "-b");
    add_option!(options.print_metadata, "-m");
    add_option!(options.print_custom, "-custom");
    add_option!(options.print_js, "-js");
    add_option!(options.print_structure, "-struct");
    add_option!(options.print_structure_text, "-struct-text");
    add_option!(options.iso_dates, "-iso-dates");
    add_option!(options.raw_dates, "-raw-dates");
    add_option!(options.print_dests, "-dests");
    add_option!(options.print_urls, "-urls");
    add_option!(
        !options.output_encoding.is_empty(),
        format!("-enc {}", options.output_encoding)
    );
    add_option!(options.print_enc, "-print-enc");
    add_option!(
        !options.owner_password.is_empty(),
        format!("-opw {}", options.owner_password)
    );
    add_option!(
        !options.user_password.is_empty(),
        format!("-upw {}", options.user_password)
    );
    add_option!(options.print_version, "-v");
    add_option!(options.print_help, "-h");

    parsed_options
}
