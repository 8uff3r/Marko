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

#[component]
fn Home() -> impl IntoView {
    let (value, set_value) = signal(0);

    view! {
        <Title text="Leptos + Tailwindcss" />
        <main>
            <div class="h-screen bg-gray-900 text-gray-100">
                <div class="h-full flex">
                    <div class="flex-1 min-w-[200px] resize-x overflow-auto border-r border-gray-700 p-4">
                        <textarea
                            class="w-full h-full bg-gray-800 text-gray-100 rounded-lg p-4 resize-none focus:outline-none focus:ring-2 focus:ring-blue-500 font-mono"
                            placeholder="Enter your text here..."
                        ></textarea>
                    </div>

                    <div class="w-2 bg-gray-800 hover:bg-blue-500 transition-colors duration-150 cursor-col-resize"></div>

                    <div class="flex-1 min-w-[200px] p-4 bg-gray-800">
                        <div class="h-full rounded-lg bg-gray-900 p-4">
                            Preview content goes here
                        </div>
                    </div>
                </div>
            </div>
        </main>
    }
}
