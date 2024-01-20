use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="tailwind" href="/output.css"/>
        // <Link rel="preload" href="https://unpkg.com/htmx.org@1.9.10" as_="script"/>
        <script
            src="https://unpkg.com/htmx.org@1.9.10"
            integrity="sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC"
            crossorigin="anonymous"
        ></script>
        <Router>
            <nav hx-boost="true">
                <a href="/">home</a>
                <a href="/blog">blog</a>
            </nav>
            <Routes>
                <Route path="/" view=Index ssr=SsrMode::Async/>
                <Route path="/blog" view=Blog ssr=SsrMode::Async/>
            </Routes>
        </Router>
    }
}

#[component]
pub fn Index() -> impl IntoView {
    view! { <div class="text-2xl">"Index"</div> }
}

#[component]
pub fn Blog() -> impl IntoView {
    view! { <div class="text-2xl">"Blog"</div> }
}
