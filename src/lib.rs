// checklist before publishing
// https://rust-lang.github.io/api-guidelines

//  mod pdf_attach;
//  mod pdf_detach;
//  mod pdf_fonts;
//  mod pdf_images;
//  mod pdf_info;
//  mod pdf_separate;
//  mod pdf_sig;
//  mod pdf_to_cairo;
//  mod pdf_to_ppm;
//  mod pdf_to_ps;
//  mod pdf_to_text;
mod pdf_to_html;
//  mod pdf_unite;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn pdf_to_html() {
        let path: &str = "test.pdf";
        let config = pdf_to_html::PdfToHtmlConfig::default();
        let result = pdf_to_html::pdf_to_html(path, config);
        //assert_eq!(result, 4);
    }
}
