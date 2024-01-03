mod api;
mod database;
mod logging;

use api::OpenMapsApi;
use color_eyre::eyre::Result;
use database::setup_database;
use dotenvy::dotenv;
use logging::install_tracing;
use poem::{Server, listener::TcpListener, Route, EndpointExt, middleware::Tracing};
use poem_openapi::{OpenApiService, ContactObject};

#[tokio::main]
async fn main() -> Result<()> {
    // install a custom error handler
    color_eyre::install()?;

    // Load environment variables from the .env file
    dotenv().ok();

    let (_guard1, _guard2) = install_tracing();

    let pool = setup_database().await?;
    let route = setup_api().data(pool).with(Tracing);

    // dual-stacking on ipv4 and ipv6
    Server::new(TcpListener::bind(":::8080")).run(route).await?;
    Ok(())
}

fn setup_api() -> Route {
    let api_service =
        OpenApiService::new(OpenMapsApi, "Open Maps Api", "0.0.1")
            .contact(ContactObject::new().name("Chirag Singla").email("chirag.singla.pi@gmail.com"))
            .server("/")
            .description(
                "Allows buyer and seller apps to use Open source maps \
                for e-commerce functionality such as creating polygon(s) of points, \
                generating motorable paths between 2 points, \
                reverse-geocoding address to point on map, \
                computing motorable distance between 2 points, \
                mapping point to polygon / path interactively / using API, etc.");
    
    let spec = api_service.spec_endpoint();
    let docs = api_service.rapidoc();

    Route::new()
        .nest("/", api_service)
        .nest("/docs", docs)
        .nest("/spec", spec)
}
