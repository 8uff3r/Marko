use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css" />
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home />
            </Routes>
        </Router>
    }
}

fn parse_string(value: String) -> String {
    let parser = pulldown_cmark::Parser::new(&value);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output.to_string()
}

#[component]
fn Home() -> impl IntoView {
    let (input_value, set_input_value) = signal("".to_string());
    let (preview_value, set_preview_value) = signal("".to_string());

    let set_values = move |value: String| {
        set_input_value.set(value.clone());
        let parsed_str = parse_string(value);
        set_preview_value.set(parsed_str);
    };

    view! {
        <Title text="Marko" />
        <main class="overflow-hidden">
            <div class="bg-gray-900 text-gray-100 h-[100svh] overflow-hidden">
                <div class="h-full flex">
                    <div class="flex-1 min-w-[200px] resize-x overflow-hidden border-r border-gray-700 p-4">
                        <textarea
                            prop:value=move || input_value.get()
                            on:input:target=move |ev| set_values(ev.target().value())
                            class="w-full h-full bg-gray-800 text-gray-100 rounded-lg p-4 resize-none focus:outline-none focus:ring-2 focus:ring-blue-500 font-mono"
                            placeholder="Enter your text here..."
                        />
                    </div>

                    <div class="w-2 bg-gray-800 hover:bg-blue-500 transition-colors duration-150 cursor-col-resize"></div>

                    <div class="flex-1 min-w-[200px] p-4 bg-gray-800">
                        <div
                            class="h-full rounded-lg bg-gray-900 p-4
                            text-gray-200 leading-relaxed text-sm sm:text-base md:text-lg
                            [&_h1]:text-2xl [&_h1]:font-bold [&_h1]:mt-6 [&_h1]:mb-3
                            [&_h2]:text-xl [&_h2]:font-semibold [&_h2]:mt-5 [&_h2]:mb-2
                            [&_h3]:text-lg [&_h3]:font-medium [&_h3]:mt-4 [&_h3]:mb-1
                            [&_p]:mb-4
                            [&_a]:text-teal-500 [&_a]:underline [&_a]:hover:text-teal-700
                            [&_strong]:font-semibold [&_strong]:text-gray-900
                            [&_code]:bg-gray-800 [&_code]:px-2 [&_code]:py-1 [&_code]:rounded [&_code]:font-mono [&_code]:text-sm
                            [&_pre]:bg-gray-800 [&_pre]:p-4 [&_pre]:rounded [&_pre]:overflow-x-auto
                            [&_pre>code]:bg-transparent [&_pre>code]:p-0
                            [&_ul]:list-disc [&_ul]:ml-6 [&_ul]:mb-4
                            [&_ol]:list-decimal [&_ol]:ml-6 [&_ol]:mb-4
                            [&_li]:mb-2
                            [&_blockquote]:border-l-4 [&_blockquote]:border-gray-300 [&_blockquote]:pl-4 [&_blockquote]:italic [&_blockquote]:mb-4
                            [&_pre]:bg-[#1E2022]
                            overflow-auto
                            "
                            inner_html=preview_value
                        ></div>
                    </div>
                </div>
            </div>
        </main>
    }
}
