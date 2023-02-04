use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "CounterStateRx")]
pub struct CounterState {
    value: i32,
}

#[auto_scope]
pub fn counter_page<G: Html>(cx: Scope, state: &CounterStateRx) -> View<G> {
    let CounterStateRx { value } = state;
    let increment = |_| value.set(*value.get() + 1);
    let decrement = |_| value.set(*value.get() - 1);
    let reset = |_| value.set(0);

    view! { cx,
        // Don't worry, there are much better ways of styling in Perseus!
        div(class = "flex flex-col justify-center align-center h-screen pt-2") {
            h1 { "I can count!" }
            p {
                "Value: "
            }
            p {
                (value.get())
            }
            button(on:click = increment, class = "btn btn-blue w-24") { "+2" }
            button(on:click = decrement, class = "btn btn-blue w-24") { "-1" }
            button(on:click = reset, class = "btn btn-blue w-24") { "Reset" }
        }
    }
}

#[engine_only_fn]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Counter" }
    }
}

#[engine_only_fn]
pub async fn get_build_state(_state: StateGeneratorInfo<()>) -> CounterState {
    CounterState { value: 0 }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("counter")
        .build_state_fn(get_build_state)
        .view_with_state(counter_page)
        .head(head)
        .build()
}
