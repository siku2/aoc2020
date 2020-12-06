use yew::prelude::*;

fn render_day(day: usize) -> Html {
    html! {
        <div>
            { day }
        </div>
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
            <div class="calendar">
                { for days }
            </div>
        }
    }
}
