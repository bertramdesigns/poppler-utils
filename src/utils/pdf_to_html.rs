#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("poppler/poppler/Object.h");
        include!("poppler/poppler/Stream.h");
        include!("poppler/poppler/Array.h");
        include!("poppler/poppler/Dict.h");
        include!("poppler/poppler/XRef.h");
        include!("poppler/poppler/Catalog.h");
        include!("poppler/poppler/Page.h");
        include!("poppler/poppler/Outline.h");
        include!("poppler/poppler/PDFDoc.h");
        include!("poppler/poppler/PDFDocFactory.h");
        include!("poppler/utils/HtmlOutputDev.h");
        include!("poppler/poppler/SplashOutputDev.h");
        include!("poppler/poppler/PDFDocEncoding.h");
        include!("poppler/poppler/DateInfo.h");
        include!("poppler/utils/Win32Console.h");
        include!("poppler/utils/InMemoryFile.h");
        include!("poppler/poppler/GlobalParams.h");
        include!("poppler/poppler/Error.h");
        include!("poppler/splash/SplashBitmap.h");
        include!("poppler/goo/gbase64.h");
        include!("poppler/goo/gbasename.h");
        include!("poppler/goo/gmem.h");
        include!("poppler/goo/GooString.h");
        include!("poppler/goo/gfile.h");

        /* Include these somehow
        #include "config.h"
        #include <poppler-config.h>
        #include <cstdio>
        #include <cstdlib>
        #include <cstddef>
        #include <cstring>
        #ifdef HAVE_DIRENT_H
        #    include <dirent.h>
        #endif
        #include <ctime>
          */

        // follow https://cxx.rs/tutorial.html
        //type BlobstoreClient;

        //fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
    }
}

pub struct PdfToHTML {
    pub(crate) pdf_doc: UniquePtr<ffi::PDFDoc>,
    pub(crate) html_output_dev: UniquePtr<ffi::HtmlOutputDev>,
    pub(crate) page_num: i32,
    pub(crate) page_count: i32,
    pub(crate) html: String,
}
