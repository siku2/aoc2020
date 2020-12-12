use crate::utils::ResultExt;
use gloo::events::EventListener;
use std::borrow::Cow;
use yew::prelude::*;

#[derive(Clone, Copy)]
pub enum Route {
    Home,
    Day(usize),
    NotFound,
}
impl Route {
    fn from_rel_path(path: &str) -> Self {
        if path.is_empty() || path == "/" {
            return Self::Home;
        } else if let Some(path) = path.strip_prefix("/day/") {
            let day = path.strip_suffix('/').unwrap_or(path);
            if let Ok(day) = day.parse() {
                return Self::Day(day);
            }
        }

        Self::NotFound
    }

    fn from_abs_path(path: &str) -> Self {
        sys::with_base_path(|base| path.strip_prefix(base))
            .map_or(Self::NotFound, Self::from_rel_path)
    }

    fn into_rel_path(self) -> Cow<'static, str> {
        match self {
            Self::Home => Cow::Borrowed("/home"),
            Self::Day(day) => Cow::Owned(format!("/day/{}", day)),
            Self::NotFound => Cow::default(),
        }
    }

    pub fn into_abs_path(self) -> String {
        sys::with_base_path(|base| format!("{}{}", base, self.into_rel_path()))
    }

    pub fn into_navigate_callback(self) -> Callback<()> {
        Callback::from(move |_| navigate_to(self))
    }
}

mod sys {
    use super::{EventListener, ResultExt};
    use wasm_bindgen::{JsCast, JsValue};
    use web_sys::{History, PopStateEvent, Url};
    use yew::Callback;

    pub fn attach_popstate_callback(cb: Callback<PopStateEvent>) -> EventListener {
        EventListener::new(&yew::utils::window(), "popstate", move |e| {
            cb.emit(e.clone().unchecked_into())
        })
    }

    fn history() -> History {
        yew::utils::window()
            .history()
            .expect("failed to get history")
    }

    pub fn push_history(url: &str) {
        history()
            .push_state_with_url(&JsValue::NULL, "", Some(url))
            .ok_or_log("failed to push to history");
    }

    pub fn abs_path() -> String {
        let location = yew::utils::window().location();
        location.pathname().expect("failed to get pathname")
    }

    fn raw_base_uri() -> String {
        yew::utils::document()
            .base_uri()
            .expect("failed to get base uri")
            .expect("base uri null")
    }

    pub fn with_base_path<T>(f: impl FnOnce(&str) -> T) -> T {
        thread_local! {
            static PATHNAME: String = {
                let mut path = Url::new(&raw_base_uri())
                    .expect("failed to create url")
                    .pathname();
                if path.ends_with('/') {
                    path.pop();
                }

                path
            };
        }

        PATHNAME.with(|path| f(&path))
    }
}

mod navigate {
    use super::Route;
    use std::{cell::RefCell, rc::Rc};
    use yew::Callback;

    thread_local! {
        // we need the second indirection for equality
        static LISTENERS: RefCell<Vec<Rc<Callback<Route>>>> = RefCell::default();
    }

    pub struct Listener(Rc<Callback<Route>>);
    impl Listener {
        pub fn new(cb: Callback<Route>) -> Self {
            let sub = Rc::new(cb);
            LISTENERS.with(|listeners| listeners.borrow_mut().push(Rc::clone(&sub)));
            Self(sub)
        }
    }
    impl Drop for Listener {
        fn drop(&mut self) {
            let sub = &self.0;
            let ok = LISTENERS.with(|listeners| {
                let mut listeners = listeners.borrow_mut();
                listeners
                    .iter()
                    .position(|other_sub| sub == other_sub)
                    .map_or(false, |i| {
                        listeners.remove(i);
                        true
                    })
            });
            if !ok {
                weblog::console_error!("failed to unsubscribe from navigation");
            }
        }
    }

    pub fn for_each_listener(mut f: impl FnMut(&Callback<Route>)) {
        LISTENERS.with(|listeners| listeners.borrow().iter().for_each(|sub| f(&**sub)));
    }
}

pub struct Subscription {
    popstate: EventListener,
    navigate: navigate::Listener,
}

pub fn subscribe(cb: Callback<Route>) -> Subscription {
    let popstate = sys::attach_popstate_callback(cb.reform(|_| get_current_route()));
    let navigate = navigate::Listener::new(cb);
    Subscription { popstate, navigate }
}

pub fn navigate_to(route: Route) {
    sys::push_history(&route.into_abs_path());
    navigate::for_each_listener(|cb| cb.emit(route));
}

pub fn get_current_route() -> Route {
    Route::from_abs_path(&sys::abs_path())
}
