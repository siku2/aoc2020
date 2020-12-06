#![warn(clippy::pedantic)]
#![allow(dead_code, clippy::cast_possible_truncation)]

use yew::prelude::*;

mod days;

struct Model;
impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        todo!()
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        todo!()
    }
}

fn main() {
    yew::start_app::<Model>();
}
