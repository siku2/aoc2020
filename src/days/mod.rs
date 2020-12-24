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
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;

pub const AVAILABLE_DAYS: usize = 17;

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
