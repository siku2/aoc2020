#![warn(clippy::pedantic)]
// `needless_lifetimes` currently needed because the rules are different between stable and nightly
#![allow(
    dead_code,
    clippy::needless_lifetimes
)]

use yew::prelude::*;

mod components;
mod days;
mod pages;
mod router;
mod services;
mod utils;

use router::Router;

struct Model;
impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{ "Advent of Code 2020" }</h1>
                <Router />
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
