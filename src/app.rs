use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container mx-auto px-4 h-screen flex">
            <div class="grid m-auto">
                <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
                <h1>{ "Hello World!" }</h1>
                <span class="subtitle">{ "from Yew with " }</span>
            </div>
        </main>
    }
}
