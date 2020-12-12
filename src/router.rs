use crate::{
    pages::{home::Home, puzzle_unavailable::PuzzleUnavailable},
    services::routing::{self, Route},
};
use yew::prelude::*;

pub enum Msg {
    RouteChanged(Route),
}

pub struct Router {
    route: Route,
    subscription: routing::Subscription,
}
impl Component for Router {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route = routing::get_current_route();
        let subscription = routing::subscribe(link.callback(Msg::RouteChanged));

        Self {
            route,
            subscription,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChanged(new_route) => {
                self.route = new_route;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.route {
            Route::Home => {
                html! { <Home /> }
            }
            Route::Day(day) => crate::days::render_day(day).unwrap_or_else(|| {
                html! { <PuzzleUnavailable day=day /> }
            }),
            Route::NotFound => {
                html! { "404" }
            }
        }
    }
}
