use crate::components::{Footer, Header, NavBar};
use crate::pages::{Docs, Home};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/docs")]
    Docs,
}

fn router(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Docs => html! { <Docs /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <NavBar />
            <Switch<Route> render={router} /> // <- must be child of <BrowserRouter>
            <Footer />
        </BrowserRouter>
    }
}
