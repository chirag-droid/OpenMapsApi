use crate::leaflet_yew::MapContext;

use js_sys::{Array, Object};
use leaflet::{LatLng, Map};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Controls;

    #[wasm_bindgen(method)]
    pub fn control(this: &Controls, options: &RoutingOptions) -> Routing;
}

#[wasm_bindgen]
extern "C" {
    pub type Routing;

    #[wasm_bindgen(method, getter)]
    pub fn Routing(this: &Routing) -> Controls;

    #[wasm_bindgen(method, js_name = addTo)]
    pub fn add_to(this: &Routing, map: &Map);
}

#[wasm_bindgen]
extern "C" {
    static L: Routing;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, js_name = RoutingOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type RoutingOptions;

    #[wasm_bindgen(method, getter, js_name = waypoints)]
    pub fn waypoints(this: &RoutingOptions) -> Array;

    #[wasm_bindgen(method, getter, js_name = serviceUrl)]
    pub fn service_url(this: &RoutingOptions) -> String;

    #[wasm_bindgen(method, setter, js_name = serviceUrl)]
    pub fn set_service_url(this: &RoutingOptions, val: &str);

    #[wasm_bindgen(method, setter, js_name = waypoints)]
    pub fn set_waypoints(this: &RoutingOptions, val: &Array);
}

impl RoutingOptions {
    pub fn new() -> Self {
        JsCast::unchecked_into(Object::new())
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct NavigationComponentProps {
    pub from: (f64, f64),
    pub to: (f64, f64),
    pub url: AttrValue
}

#[function_component(Navigation)]
pub fn routing_component(props: &NavigationComponentProps) -> Html {
    let NavigationComponentProps { from, to, url } = props.clone();

    let ctx = use_context::<MapContext>().unwrap();

    if let Some(map) = ctx.map.clone() {
        let options = RoutingOptions::new();
        options.set_waypoints(
            &[LatLng::new(from.0, from.1), LatLng::new(to.0, to.1)]
                .iter()
                .map(JsValue::from)
                .collect::<Array>(),
        );
        options.set_service_url(url.as_str());

        L.Routing().control(&options).add_to(&map);
    }

    html! {}
}
