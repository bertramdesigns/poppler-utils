use crate::utils::{run_program, PopplerFile};
use std::future::Future;

/*
/fix Replace the paramaters with the ones in the list below. The replaced parameters should match the formatting.

For example,
`boundingBoxXhtml: { arg: "-bbox", type: "boolean" },` should translate to `bounding_box_xhtml: bool, // boundingBoxXhtml, -bbox`

Items to translate:
```
boundingBoxXhtml: { arg: "-bbox", type: "boolean" },
boundingBoxXhtmlLayout: {arg: "-bbox-layout", type: "boolean"},
 ```
*/
pub struct PdfToTextConfig<'a> {
    bounding_box_xhtml: bool,        // bbox, -bbox
    bounding_box_xhtml_layout: bool, // bboxLayout, -bbox-layout
    crop_box: bool,                  // useCropBox, -cropbox
    h: f64,                          // h, -H
    w: f64,                          // w, -W
    x: f64,                          // x, -x
    y: f64,                          // y, -y
    eol_convention: &'a str,         // textEOLStr, -eol
    first_page_to_convert: i32,      // firstPage, -f
    fixed_width_layout: f64,         // fixedPitch, -fixed
    generate_html_meta_file: bool,   // htmlMeta, -htmlmeta
    generate_tsv_file: bool,         // tsvMode, -tsv
    last_page_to_convert: i32,       // lastPage, -l
    list_encoding_options: bool,     // printEnc, -listenc
    maintain_layout: bool,           // physLayout, -layout
    no_diagonal_text: bool,          // discardDiag, -nodiag
    no_page_breaks: bool,            // noPageBreaks, -nopgbrk
    output_encoding: &'a str,        // textEncName, -enc
    owner_password: &'a str,         // ownerPassword, -opw
    print_version_info: bool,        // printVersion, -v
    quiet: bool,                     // quiet, -q
    raw_layout: bool,                // rawOrder, -raw
    resolution: f64,                 // resolution, -r
    user_password: &'a str,          // userPassword, -upw
    print_help: bool,                // printHelp, -h, -help, --help, -?
    column_spacing: f64,             // colspacing, -colspacing
}

impl<'a> Default for PdfToTextConfig<'a> {
    fn default() -> Self {
        Self {
            bounding_box_xhtml: false, // true makes htmlMeta true
            bounding_box_xhtml_layout: false,
            crop_box: false,
            h: 0.0, // in pixels
            w: 0.0, // in pixels
            x: 0.0,
            y: 0.0,
            column_spacing: 0.7, // max 10
            eol_convention: "",  // unix, dos, mac
            first_page_to_convert: 1,
            fixed_width_layout: 0.0, // >0 makes physLayout true. In points defining the width of a single character
            generate_html_meta_file: false,
            generate_tsv_file: false, // true make also true physLayout, fixedPitch, rawOrder, htmlMeta, discardDiag
            last_page_to_convert: 0,
            list_encoding_options: false,
            maintain_layout: false,
            no_diagonal_text: false,
            no_page_breaks: false,
            output_encoding: "",
            owner_password: "",
            print_version_info: false,
            quiet: false,
            raw_layout: false,
            resolution: 72.0, // in dpi
            user_password: "",
            print_help: false,
        }
    }
}

pub fn pdf_to_text(
    file: PopplerFile,
    options: PdfToTextConfig<'static>,
) -> impl Future<Output = Result<String, std::io::Error>> {
    let parsed_options = parse_options(&options);

    // return the non-awaited future
    run_program(file, "pdftotext", parsed_options)
}

fn parse_options(options: &PdfToTextConfig) -> Vec<String> {
    let mut parsed_options = Vec::new();

    macro_rules! add_option {
        ($condition:expr, $arg:expr) => {
            if $condition {
                parsed_options.push($arg.to_string());
            }
        };
    }
    add_option!(options.bounding_box_xhtml, "-bbox");
    add_option!(options.bounding_box_xhtml_layout, "-bbox-layout");
    add_option!(options.crop_box, "-cropbox");
    add_option!(options.h != 0.0, format!("-H {}", options.h));
    add_option!(options.w != 0.0, format!("-W {}", options.w));
    add_option!(options.x != 0.0, format!("-x {}", options.x));
    add_option!(options.y != 0.0, format!("-y {}", options.y));
    add_option!(
        !options.eol_convention.is_empty(),
        format!("-eol {}", options.eol_convention)
    );
    add_option!(
        options.first_page_to_convert != 1,
        format!("-f {}", options.first_page_to_convert)
    );
    add_option!(
        options.fixed_width_layout != 0.0,
        format!("-fixed {}", options.fixed_width_layout)
    );
    add_option!(options.generate_html_meta_file, "-htmlmeta");
    add_option!(options.generate_tsv_file, "-tsv");
    add_option!(
        options.last_page_to_convert != 0,
        format!("-l {}", options.last_page_to_convert)
    );
    add_option!(options.list_encoding_options, "-listenc");
    add_option!(options.maintain_layout, "-layout");
    add_option!(options.no_diagonal_text, "-nodiag");
    add_option!(options.no_page_breaks, "-nopgbrk");
    add_option!(
        !options.output_encoding.is_empty(),
        format!("-enc {}", options.output_encoding)
    );
    add_option!(
        !options.owner_password.is_empty(),
        format!("-opw {}", options.owner_password)
    );
    add_option!(options.print_version_info, "-v");
    add_option!(options.quiet, "-q");
    add_option!(options.raw_layout, "-raw");
    add_option!(
        options.resolution != 72.0,
        format!("-r {}", options.resolution)
    );
    add_option!(
        !options.user_password.is_empty(),
        format!("-upw {}", options.user_password)
    );
    add_option!(options.print_help, "-h");
    add_option!(
        options.column_spacing != 0.7,
        format!("-colspacing {}", options.column_spacing)
    );

    parsed_options
}
