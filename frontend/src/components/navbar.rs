use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
        <nav class="bg-gray-200 p-4">
            <ul class="flex space-x-4">
                <li>
                <Link<Route> to={Route::Home} classes="hover:underline">
                    { "Home" }
                </Link<Route>>
                </li>

                <li>
                <Link<Route> to={Route::Docs} classes="hover:underline">
                    { "Documentation" }
                </Link<Route>>
                </li>
            </ul>
        </nav>
    }
}
