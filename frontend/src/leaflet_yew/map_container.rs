use std::rc::Rc;

use leaflet::{LatLng, Map, MapOptions};
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Debug, Clone)]
pub struct MapReducible {
    pub map: Option<Map>,
    ctr: u32,
}

impl Reducible for MapReducible {
    type Action = Map;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        MapReducible {
            map: Some(action),
            ctr: self.ctr + 1,
        }
        .into()
    }
}

impl PartialEq for MapReducible {
    fn eq(&self, other: &Self) -> bool {
        self.ctr == other.ctr
    }
}

pub type MapContext = UseReducerHandle<MapReducible>;

/// Props for [`MapContainer`]
#[derive(PartialEq, Properties, Clone)]
pub struct MapContainerProps {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    pub center: (f64, f64),
}

#[function_component(MapContainer)]
pub fn map_container(props: &MapContainerProps) -> Html {
    let MapContainerProps {
        children,
        class,
        center,
    } = props.clone();

    let ctx = use_reducer_eq(|| MapReducible { map: None, ctr: 0 });

    if let Some(map) = ctx.map.clone() {
        map.set_view(&LatLng::new(center.0, center.1), 11.0);
    }

    let div_ref = use_node_ref();

    // Attach map to the component
    {
        let div_ref = div_ref.clone();
        let ctx = ctx.clone();

        use_effect_with(div_ref, move |div_ref| {
            let div = div_ref
                .cast::<HtmlElement>()
                .expect("Div ref not attached to div element");
            let map = Map::new_with_element(&div, &MapOptions::default());

            ctx.dispatch(map);
        });
    }

    // Detach map from the component
    {
        let ctx = ctx.clone();

        use_effect_with((), move |_| {
            move || {
                if let Some(map) = ctx.map.clone() {
                    map.remove();
                }
            }
        })
    }

    html!(
        <div ref={div_ref} {class}>
            <ContextProvider<MapContext> context={ctx}>
                {children}
            </ContextProvider<MapContext>>
        </div>
    )
}
