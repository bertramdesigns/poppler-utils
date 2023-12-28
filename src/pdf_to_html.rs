use crate::utils::{run_program, PopplerFile};
use std::future::Future;

pub struct PdfToHtmlConfig<'a> {
    output_path: &'a str,         // this package only
    first_page_to_convert: i32,   // firstPage, -f
    last_page_to_convert: i32,    // lastPage, -l
    quiet: bool,                  // errQuiet, -q
    raw_order: bool,              // rawOrder, -raw
    print_commands: bool,         // printCommands, -c
    print_help: bool,             // printHelp, -h
    exchange_pdf_links: bool,     // printHtml, -p
    complex_output: bool,         // complexMode, -c
    single_page: bool,            // singleHtml, -s
    data_urls: bool,              // dataUrls, -dataurls
    ignore_images: bool,          // ignore, -i
    no_frames: bool,              // noFrames, -noframes
    stdout: bool,                 // stout, -stdout
    zoom: f64,                    // scale, -zoom
    xml_output: bool,             // xml, -xml
    no_rounded_coordinates: bool, // noRoundedCoordinates, -noroundcoord
    extract_hidden: bool,         // showHidden, -hidden
    no_merge_paragraph: bool,     // noMerge, -nomerge
    output_encoding: &'a str,     // textEncName, -enc
    image_format: &'a str,        // extension, -fmt
    print_version_info: bool,     // printVersion, -v
    owner_password: &'a str,      // ownerPassword, -opw
    user_password: &'a str,       // userPassword, -upw
    no_drm: bool,                 // noDrm, -nodrm
    word_break_threshold: f64,    // wordBreakThreshold, -wbt
    font_full_name: bool,         // fontFullName, -fontfullname
}

impl<'a> Default for PdfToHtmlConfig<'a> {
    fn default() -> Self {
        Self {
            output_path: "", // directory to write files to
            first_page_to_convert: 1,
            last_page_to_convert: 0, // 0 means all pages
            raw_order: true,         // not exposed in node-poppler
            print_commands: true,    // not exposed in node-poppler
            print_help: false,       // not exposed in node-poppler
            quiet: false,
            exchange_pdf_links: false, // swaps .pdf ending for .html
            complex_output: false,     // true makes stdout false
            single_page: false,
            data_urls: false,
            ignore_images: false,
            no_frames: false,
            stdout: false,     // true makes complexOutput false, noFrames true
            zoom: 1.5,         // max 3.0, min 0.5; 1 = 72dpi
            xml_output: false, // true makes complexOutput true, singlePage false, noFrames true, noMergeParagraph true
            no_rounded_coordinates: false, // xml only
            extract_hidden: false,
            no_merge_paragraph: false,
            output_encoding: "", // defaults to "UTF-8" and seems to support UTF-16, Latin1, ASCII7, Symbol, ZapfDingbats, (UCS-4?)
            image_format: "png", // png, jpg (default png)
            print_version_info: false,
            owner_password: "", // max 32 characters
            user_password: "",  // max 32 characters
            no_drm: false,
            word_break_threshold: 10.0, // default 10.0 (10%)
            font_full_name: false,
        }
    }
}

pub fn pdf_to_html(
    file: PopplerFile,
    options: PdfToHtmlConfig<'static>,
) -> impl Future<Output = Result<String, std::io::Error>> {
    let parsed_options = parse_options(&options);

    // return the non-awaited future
    run_program(file, "pdftohtml", parsed_options)
}

fn parse_options(options: &PdfToHtmlConfig) -> Vec<String> {
    let mut parsed_options = Vec::new();

    macro_rules! add_option {
        ($condition:expr, $arg:expr) => {
            if $condition {
                parsed_options.push($arg.to_string());
            }
        };
    }
    add_option!(
        options.first_page_to_convert != 1,
        format!("-f {}", options.first_page_to_convert)
    );
    add_option!(
        options.last_page_to_convert != 0,
        format!("-l {}", options.last_page_to_convert)
    );
    add_option!(options.quiet, "-q");
    add_option!(options.exchange_pdf_links, "-p");
    add_option!(options.single_page, "-s");
    add_option!(options.raw_order, "-raw");
    add_option!(options.print_commands, "-c");
    add_option!(options.print_help, "-h");
    add_option!(options.complex_output, "-c");
    add_option!(options.data_urls, "-dataurls");
    add_option!(options.ignore_images, "-i");
    add_option!(
        options.image_format != "png",
        format!("-fmt {}", options.image_format)
    );
    add_option!(options.zoom != 1.5, format!("-zoom {}", options.zoom));
    add_option!(options.no_frames, "-noframes");
    add_option!(options.stdout, "-stdout");
    add_option!(options.xml_output, "-xml");
    add_option!(options.no_rounded_coordinates, "-noroundcoord");
    add_option!(options.extract_hidden, "-hidden");
    add_option!(options.no_merge_paragraph, "-nomerge");
    add_option!(
        !options.output_encoding.is_empty(),
        format!("-enc {}", options.output_encoding)
    );
    add_option!(options.print_version_info, "-v");
    add_option!(
        !options.owner_password.is_empty(),
        format!("-opw {}", options.owner_password)
    );
    add_option!(
        !options.user_password.is_empty(),
        format!("-upw {}", options.user_password)
    );
    add_option!(options.no_drm, "-nodrm");
    add_option!(
        options.word_break_threshold != 10.0,
        format!("-wbt {}", options.word_break_threshold)
    );
    add_option!(options.font_full_name, "-fontfullname");
    add_option!(!options.output_path.is_empty(), options.output_path);

    parsed_options
}
