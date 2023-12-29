// checklist before publishing
// https://rust-lang.github.io/api-guidelines

// pub mod pdf_attach;
// pub mod pdf_detach;
// pub mod pdf_fonts;
pub mod pdf_images;
pub mod pdf_info;
pub mod pdf_separate;
// pub mod pdf_sig;
// pub mod pdf_to_cairo;
// pub mod pdf_to_ppm;
// pub mod pdf_to_ps;
pub mod pdf_to_html;
pub mod pdf_to_text;
pub mod pdf_unite;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::pdf_info::{pdf_info, PdfInfoConfig};
    use crate::pdf_to_html::{pdf_to_html, PdfToHtmlConfig};
    use crate::pdf_to_text::{pdf_to_text, PdfToTextConfig};
    use crate::utils::AsPopplerPath;
    use tokio::runtime::Builder;

    // #[test]
    // fn pdf_to_html_works() {
    //     let file_path = "/test.pdf";
    //     let file = file_path.as_poppler_path();
    //     let config = pdf_to_html::PdfToHtmlConfig::default();
    //     let _result = pdf_to_html::pdf_to_html(file, config);
    //     //assert_eq!(result, 4);
    // }

    #[test]
    fn pdf_info_works() {
        run_test(async {
            let mut file_path = std::env::current_dir().unwrap();
            file_path.push("./src/test.pdf");
            // println!("file_path: {:?}", file_path);

            let file = file_path.as_poppler_path();
            let config = PdfInfoConfig::default();
            // config.print_help = true;

            let _result = pdf_info(file, config).await;

            //assert_eq!(1, 4);
        })
    }

    #[test]
    fn pdf_to_html_works() {
        run_test(async {
            let mut file_path = std::env::current_dir().unwrap();
            file_path.push("./src/test.pdf");
            // println!("file_path: {:?}", file_path);

            let file = file_path.as_poppler_path();
            let config = PdfToHtmlConfig::default();
            // config.print_help = true;

            let _result = pdf_to_html(file, config).await;

            //assert_eq!(1, 4);
        })
    }

    #[test]
    fn pdf_to_text_works() {
        run_test(async {
            let mut file_path = std::env::current_dir().unwrap();
            file_path.push("./src/test.pdf");
            // println!("file_path: {:?}", file_path);

            let file = file_path.as_poppler_path();
            let config = PdfToTextConfig::default();
            // config.print_help = true;

            let _result = pdf_to_text(file, config).await;

            //assert_eq!(1, 4);
        })
    }

    // TODO: simplify by using #[tokio::test] if no setup/teardown is needed in the future
    // https://lik.ai/blog/async-setup-and-teardown-in-rust
    fn run_test<T>(test: T) -> ()
    where
        T: std::future::Future + std::panic::UnwindSafe,
    {
        // setup();

        let result = std::panic::catch_unwind(|| {
            Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(test);
        });

        // teardown();

        assert!(result.is_ok())
    }
}
