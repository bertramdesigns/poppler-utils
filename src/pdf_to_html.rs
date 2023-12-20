// use camino::Utf8PathBuf;
// use std::fs::File;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
#[allow(dead_code)]
pub struct PdfToHtmlConfig {
    first_page: i32,
    last_page: i32,
    raw_order: bool,
    print_commands: bool,
    print_help: bool,
    print_html: bool,
    complex_mode: bool,
    single_html: bool,
    data_urls: bool,
    ignore: bool,
    extension: String,
    scale: f64,
    no_frames: bool,
    stout: bool,
    xml: bool,
    no_rounded_coordinates: bool,
    err_quiet: bool,
    no_drm: bool,
    word_break_threshold: f64,
    show_hidden: bool,
    no_merge: bool,
    font_full_name: bool,
    owner_password: String,
    user_password: String,
    print_version: bool,
    text_enc_name: String,
}

impl Default for PdfToHtmlConfig {
    fn default() -> Self {
        Self {
            first_page: 1,
            last_page: 0,
            raw_order: true,
            print_commands: true,
            print_help: false,
            print_html: false,
            complex_mode: false,
            single_html: false,
            data_urls: false,
            ignore: false,
            extension: String::from("png"),
            scale: 1.5,
            no_frames: false,
            stout: false,
            xml: false,
            no_rounded_coordinates: false,
            err_quiet: false,
            no_drm: false,
            word_break_threshold: 0.1,
            show_hidden: false,
            no_merge: false,
            font_full_name: false,
            owner_password: String::new(),
            user_password: String::new(),
            print_version: false,
            text_enc_name: String::new(),
        }
    }
}

pub trait AsPath {
    fn as_path(&self) -> &Path;
}

impl AsPath for Path {
    fn as_path(&self) -> &Path {
        self
    }
}

impl AsPath for &str {
    fn as_path(&self) -> &Path {
        Path::new(self)
    }
}

#[allow(dead_code)]
pub fn pdf_to_html<T: AsPath>(file_location: T, options: PdfToHtmlConfig) -> () {
    // get the proper executable for the current operating system (ELF, Mach-O, PE)
    let os = std::env::consts::OS;

    // determine which of 3 executable folders to use
    let path_to_executable = match os {
        "windows" => "src/poppler/win/bin/pdftohtml.exe",
        "macos" => "src/poppler/mac/bin/pdftohtml",
        "ios" => "src/poppler/mac/bin/pdftohtml",
        _ => "src/poppler/unix/bin/pdftohtml",
    };

    // get absolute path of this root directory
    let root = std::env::current_dir().unwrap();
    let exe_path = root.join(path_to_executable.as_path());

    // TODO: if file_location is undefined and not a valid pdf file then error and return
    //let data = std::fs::read(&file_location.as_path());

    let parsed_options = parse_options(&file_location.as_path(), &options);

    let mut handle = Command::new(exe_path);
    //handle.args(parsed_options);
    handle.arg("-h");
    let result = handle.output();

    if result.is_ok() {
        let output = result.unwrap();
        match String::from_utf8(output.stderr) {
            Ok(stderr) => println!("Execution was successful: {}", stderr),
            Err(e) => println!("Invalid UTF-8 sequence: {}", e),
        }
        println!("parsed_options: {:?}", parsed_options);
    } else {
        println!("error: {}", result.err().unwrap());
    }
}

fn parse_options(file_location: &Path, options: &PdfToHtmlConfig) -> Vec<String> {
    let mut parsed_options = Vec::new();

    macro_rules! add_option {
        ($condition:expr, $arg:expr) => {
            if $condition {
                parsed_options.push($arg.to_string());
            }
        };
    }
    add_option!(file_location.exists(), file_location.to_str().unwrap());
    add_option!(
        options.first_page != 1,
        format!("-f {}", options.first_page)
    );
    add_option!(options.last_page != 0, format!("-l {}", options.last_page));
    add_option!(options.raw_order, "-raw");
    add_option!(options.print_commands, "-c");
    add_option!(options.print_help, "-h");
    add_option!(options.print_html, "-s");
    add_option!(options.complex_mode, "-c");
    add_option!(options.single_html, "-s");
    add_option!(options.data_urls, "-dataurls");
    add_option!(options.ignore, "-i");
    add_option!(
        options.extension != "png",
        format!("-fmt {}", options.extension)
    );
    add_option!(options.scale != 1.5, format!("-zoom {}", options.scale));
    add_option!(options.no_frames, "-noframes");
    add_option!(options.stout, "-stdout");
    add_option!(options.xml, "-xml");
    add_option!(options.no_rounded_coordinates, "-noroundcoord");
    add_option!(options.err_quiet, "-q");
    add_option!(options.no_drm, "-nodrm");
    add_option!(
        options.word_break_threshold != 0.1,
        format!("-wbt {}", options.word_break_threshold)
    );
    add_option!(options.show_hidden, "-hidden");
    add_option!(options.no_merge, "-nomerge");
    add_option!(options.font_full_name, "-fontfullname");

    // first_page: 1,
    // last_page: 0,
    // raw_order: true,
    // print_commands: true,
    // print_help: false,
    // print_html: false,
    // complex_mode: false,
    // single_html: false,
    // data_urls: false,
    // ignore: false,
    // extension: String::from("png"),
    // scale: 1.5,
    // no_frames: false,
    // stout: false,
    // xml: false,
    // no_rounded_coordinates: false,
    // err_quiet: false,
    // no_drm: false,
    // word_break_threshold: 0.1,
    // show_hidden: false,
    // no_merge: false,
    // font_full_name: false,
    // owner_password: String::new(),
    // user_password: String::new(),
    // print_version: false,
    // text_enc_name: String::new(),

    parsed_options
}
