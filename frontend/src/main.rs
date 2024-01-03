use crate::components::{
    control::{Cities, Control},
    map_component::{City, MapComponent, Point},
};
use yew::prelude::*;
mod components;

enum Msg {
    SelectCity(City),
}

struct Model {
    city: City,
    cities: Cities,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let delhi = City {
            name: "Delhi".to_string(),
            lat: Point(28.70405920, 77.10249020),
        };
        let gurugram = City {
            name: "Gurugram".to_string(),
            lat: Point(28.43891000, 77.00592000),
        };
        let cities: Cities = Cities {
            list: vec![delhi, gurugram],
        };
        let city = cities.list[0].clone();
        Self { city, cities }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectCity(city) => {
                self.city = self
                    .cities
                    .list
                    .iter()
                    .find(|c| c.name == city.name)
                    .unwrap()
                    .clone();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cb = ctx.link().callback(Msg::SelectCity);
        html! {
            <>
                <MapComponent city={&self.city}  />
                <Control select_city={cb} cities={&self.cities}/>
            </>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}