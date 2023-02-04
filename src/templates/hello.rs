use perseus::{engine_only_fn, template::Template};
use sycamore::prelude::*;
use sycamore::suspense::Suspense;

#[cfg(target_arch = "wasm32")]
use crate::api;

pub fn hello_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        sycamore::web::NoSsr {
            Suspense(fallback = view! { cx, "Loading..." }) {
                Hello {}
            }
        }
        a(href = "counter", id = "counter-link") { "Counter!" }
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
pub async fn Hello<G: Html>(cx: Scope<'_>) -> View<G> {
    use std::rc::Rc;
    use sycamore_query::{prelude::*, QueryClient};
    provide_context(cx, Rc::new(QueryClient::default()));

    let name = create_rc_signal("World".to_string());
    let name_text = create_ref(cx, name.clone());

    let Query { data, refetch, .. } = use_query(cx, ("hello", name.clone().rc_key()), move || {
        api::call_hello("hello")
    });

    let Mutation { mutate, .. } = use_mutation(
        cx,
        |name: String| async { Result::<_, String>::Ok(name) },
        move |client, _| client.invalidate_queries(keys!["hello"]),
    );

    view! { cx,
        input(bind:value = name_text, class = "text-field")
        button(class = "btn btn-blue", on:click = move |_| refetch()) { "Update" }
        button(class = "btn", on:click = move |_| mutate("World".to_string())) { "Invalidate" }
        (match data.get_data() {
            QueryData::Ok(message) => view! { cx, p { (message) } },
            QueryData::Err(err) => view! { cx, p { (err) } },
            QueryData::Loading => view! { cx, p { "Loading..." } }
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[component]
pub async fn Hello<G: Html>(cx: Scope<'_>) -> View<G> {
    view! { cx, }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("hello").view(hello_page).head(head).build()
}

#[engine_only_fn]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Index Page | Perseus Example – Basic" }
    }
}
