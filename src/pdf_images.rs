use crate::utils::{run_program, PopplerFile};
use std::future::Future;

pub struct PdfImagesConfig<'a> {
    pub first_page: i32,         // firstPage, -f
    pub last_page: i32,          // lastPage, -l
    pub all_files: bool,         // allFiles, -all
    pub ccitt_file: bool,        // ccittFile, -ccitt
    pub list: bool,              // list, -list
    pub jbig2_file: bool,        // jbig2File, -jbig2
    pub jpeg2000_file: bool,     // jpeg2000File, -jp2
    pub jpeg_file: bool,         // jpegFile, -j
    pub png_file: bool,          // pngFile, -png
    pub tiff_file: bool,         // tiffFile, -tiff
    pub owner_password: &'a str, // ownerPassword, -opw
    pub user_password: &'a str,  // userPassword, -upw
    pub print_version: bool,     // printVersion, -v
    pub print_help: bool,        // printHelp, -h
}

impl<'a> Default for PdfImagesConfig<'a> {
    fn default() -> Self {
        Self {
            first_page: 1,
            last_page: 0,
            all_files: false,
            ccitt_file: false,
            list: false,
            jbig2_file: false,
            jpeg2000_file: false,
            jpeg_file: false,
            png_file: false,
            tiff_file: false,
            owner_password: "",
            user_password: "",
            print_version: false,
            print_help: false,
        }
    }
}

pub fn pdf_image(
    file: PopplerFile,
    options: PdfImagesConfig<'static>,
) -> impl Future<Output = Result<String, std::io::Error>> {
    let parsed_options = parse_options(&options);

    // return the non-awaited future
    run_program(file, "pdfimages", parsed_options)
}

fn parse_options(options: &PdfImagesConfig) -> Vec<String> {
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
    add_option!(options.all_files, "-all");
    add_option!(options.ccitt_file, "-ccitt");
    add_option!(options.list, "-list");
    add_option!(options.jbig2_file, "-jbig2");
    add_option!(options.jpeg2000_file, "-jp2");
    add_option!(options.jpeg_file, "-j");
    add_option!(options.png_file, "-png");
    add_option!(options.tiff_file, "-tiff");
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
