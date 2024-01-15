use yew::prelude::*;

use crate::leaflet_yew::{MapContainer, Navigation, TileComponent};

#[function_component(Home)]
pub fn home() -> Html {
    let tile_service = std::option_env!("TILE_SERVER").unwrap_or("/tile");

    let routing_service =
        std::option_env!("ROUTING_SERVER").unwrap_or("https://router.project-osrm.org/route/v1");

    html! {
        <main class="container mx-auto mt-4 p-4">
        <MapContainer class="h-96" center={(28.43891000, 77.00592000)}>
            <TileComponent url={tile_service} />
            <Navigation url={routing_service} from={(28.43891000, 77.00592000)} to={(28.494976, 77.089542)} />
        </MapContainer>

        <div class="bg-white p-4 shadow-md rounded-md mt-4">
            <h2 class="text-lg font-semibold">{"Reverse Geocode"}</h2>
            <p class="text-gray-600">{"Find location details by coordinates."}</p>
            <button class="bg-blue-500 text-white px-4 py-2 mt-2 rounded-md hover:bg-blue-600">{"Try it Now"}</button>
        </div>


        <div class="mt-4">
            <h2 class="text-xl font-semibold">{"API Documentation"}</h2>
            <p class="text-gray-600">{"Explore the API using the OpenAPI specification."}</p>
            <a href="link-to-your-openapi-docs" target="_blank" class="text-blue-500 hover:underline">{"View API Docs"}</a>
        </div>
        </main>
    }
}
