mod api;
mod error_pages;
mod templates;

#[cfg(not(target_arch = "wasm32"))]
use perseus::{i18n::TranslationsManager, stores::MutableStore};
use perseus::{plugins::Plugins, prelude::*};
#[cfg(engine)]
use perseus::{server::ServerOptions, turbine::Turbine};
use perseus_compress::CompressionOptions;
use sycamore::view;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[perseus::main(axum_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .error_views(error_pages::get_error_views())
        .template(crate::templates::index::get_template())
        .template(templates::counter::get_template())
        .template(templates::hello::get_template())
        .index_view(|cx| {
            #[cfg(target_arch = "wasm32")]
            wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
            view! { cx,
                html {
                    head {
                        meta(charset = "UTF-8")
                        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
                        link(rel = "stylesheet", href = "/static/tailwind.css")
                    }
                    body {
                        PerseusRoot()
                    }
                }
            }
        })
        .plugins(
            Plugins::new()
                .plugin(
                    perseus_tailwind::get_tailwind_plugin,
                    perseus_tailwind::TailwindOptions {
                        in_file: "src/tailwind.css".into(),
                        out_file: "dist/static/tailwind.css".into(),
                    },
                )
                .plugin(
                    perseus_compress::get_compression_plugin,
                    CompressionOptions {
                        should_run: cfg!(not(debug_assertions)),
                        ..CompressionOptions::default()
                    },
                ),
        )
        .static_alias("/static/tailwind.css", "dist/static/tailwind.css")
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn axum_server<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    turbine: &'static Turbine<M, T>,
    props: ServerOptions,
    (host, port): (String, u16),
) {
    use std::net::SocketAddr;

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address provided to bind to.");

    let app = perseus_axum::get_router(turbine, props)
        .await
        .nest("/api", api::routes());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
