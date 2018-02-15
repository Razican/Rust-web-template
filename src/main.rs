//! Web launcher.

#![feature(plugin)]
#![plugin(rocket_codegen)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy))]
#![forbid(anonymous_parameters)]
//#![cfg_attr(feature = "cargo-clippy", warn(clippy_pedantic))]
#![deny(variant_size_differences, unused_results, unused_qualifications, unused_import_braces,
        unsafe_code, trivial_numeric_casts, trivial_casts, missing_docs,
        missing_debug_implementations, missing_copy_implementations, box_pointers,
        unused_extern_crates)]

extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
extern crate web_core;

use rocket_contrib::Template;
use web_core::*;

/// Program entry point.
#[allow(box_pointers)]
fn main() {
    let _ = dotenv::dotenv().ok();

    let server = rocket::ignite()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                image,
                favicon,
                android_config,
                windows_config,
                favicons,
                css,
                js,
                homepage,
            ],
        )
        .mount(
            "api/v1",
            routes![api::v1::oauth::refresh_token, api::v1::oauth::access_token,],
        );

    #[cfg(feature = "source_maps")]
    let error = {
        server
            .mount("/", routes![source_maps, js_source_maps])
            .launch()
    };

    #[cfg(not(feature = "source_maps"))]
    let error = { server.launch() };

    panic!("Launch failed! Error: {}", error);
}
