use leaflet::TileLayer;
use yew::prelude::*;

use crate::leaflet_yew::MapContext;

#[derive(Properties, Clone, PartialEq)]
pub struct TileComponentProps {
    pub url: AttrValue,
}

#[function_component(TileComponent)]
pub fn tile_component(props: &TileComponentProps) -> Html {
    let TileComponentProps { url } = props.clone();

    let ctx = use_context::<MapContext>().unwrap();

    if let Some(map) = ctx.map.clone() {
        let tile_layer = TileLayer::new(format!("{url}/{{z}}/{{x}}/{{y}}.png").as_str());
        tile_layer.add_to(&map);
    }

    html! {}
}
