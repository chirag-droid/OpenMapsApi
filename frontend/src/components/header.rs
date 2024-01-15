use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html!(
        <header class="bg-blue-500 p-4 text-white">
            <h1 class="text-2xl font-bold">{"OpenStreetData API"}</h1>
        </header>
    )
}
