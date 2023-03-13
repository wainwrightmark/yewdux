use std::rc::Rc;

use yew::prelude::*;
use yewdux::{log, prelude::*};

use serde::{Deserialize, Serialize};

struct LogListener1;
impl Listener for LogListener1 {
    type Store = State;

    fn on_change(&mut self, state: Rc<Self::Store>) {
        log::info!("Listener 1: count: {}", state.count);
    }
}

struct LogListener2;
impl Listener for LogListener2 {
    type Store = State;

    fn on_change(&mut self, state: Rc<Self::Store>) {
        log::info!("Listener 2: count: {}", state.count);
    }
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct State {
    count: u32,
}

impl Store for State {
    fn new() -> Self {
        init_listener(LogListener1);
        init_listener(LogListener2);
        Self { count: 0 }
    }

    fn should_notify(&self, other: &Self) -> bool {
        self != other
    }
}

#[function_component]
fn App() -> Html {
    let (state, dispatch) = use_store::<State>();
    let onclick = dispatch.reduce_mut_callback(|state| state.count += 1);

    html! {
        <>
        <p>{ state.count }</p>
        <button {onclick}>{"+1"}</button>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
