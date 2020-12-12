mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;

pub const AVAILABLE_DAYS: usize = 12;

pub fn render_day(day: usize) -> Option<yew::Html> {
    macro_rules! builder {
        ($day:ident, $( $num:literal => $module:path, )+) => {
            match $day {
                $(
                    $num => Some(::yew::html! { <$module::Page /> }),
                )*
                _ => None,
            }
        }
    }

    builder! {day,
        1 => d01,
    }
}
