use perseus::{engine_only_fn, prelude::*, ReactiveState};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexPageStateRx")]
pub struct IndexPageState {
    greeting: String,
}

#[auto_scope]
pub fn index_page<G: Html>(cx: Scope, state: &IndexPageStateRx) -> View<G> {
    view! { cx,
        p { (state.greeting.get()) }
        a(href = "counter", id = "counter-link") { "Counter!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head(head)
        .build()
}

#[engine_only_fn]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Index Page | Perseus Example – Basic" }
    }
}

#[engine_only_fn]
pub async fn get_build_state(_state: StateGeneratorInfo<()>) -> IndexPageState {
    IndexPageState {
        greeting: "Hello Lisa!".to_string(),
    }
}
