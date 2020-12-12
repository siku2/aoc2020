use crate::services::routing::Route;
use yew::events::MouseEvent;
use yew::prelude::*;

fn render_day(day: usize) -> Html {
    let route = Route::Day(day);
    let onclick = route.into_navigate_callback().reform(|e: MouseEvent| {
        e.prevent_default();
    });
    html! {
        <a href=route.into_abs_path() onclick=onclick>
            { day }
        </a>
    }
}

pub struct Calendar;
impl Component for Calendar {
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
        let days = (1..=25).map(render_day);
        html! {
            <div>
                { for days }
            </div>
        }
    }
}
