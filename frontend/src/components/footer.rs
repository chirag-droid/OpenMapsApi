use yew::prelude::*;

#[function_component(Footer)]
pub fn header() -> Html {
    html! {
        <footer class="bg-gray-200 p-4 mt-4">
            <p class="text-gray-600 text-sm">{"© 2024 Open Maps Api. All rights reserved."}</p>
        </footer>
    }
}
