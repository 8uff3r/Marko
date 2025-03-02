use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Define the JavaScript function we'll call.
    #[wasm_bindgen(js_name = "downloadHtmlAsPdf")]
    pub fn download_html_as_pdf(element_id: &str); // Pass elementId as string
}

pub fn set_panic_hook() {
    // When the app panics, show the error message in the browser's console.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn document() -> web_sys::Document {
    web_sys::window()
        .expect("no global `window` exists")
        .document()
        .expect("should have a document on window")
}

pub fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}
