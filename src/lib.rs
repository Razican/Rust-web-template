//! Web core.

#![recursion_limit = "128"]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy))]
#![forbid(anonymous_parameters)]
//#![cfg_attr(feature = "cargo-clippy", warn(clippy_pedantic))]
#![deny(variant_size_differences, unused_results, unused_qualifications, unused_import_braces,
        unsafe_code, trivial_numeric_casts, trivial_casts, missing_docs,
        missing_debug_implementations, missing_copy_implementations, box_pointers,
        unused_extern_crates)]
#![allow(unused_imports, unused_extern_crates)]

#[macro_use]
extern crate failure;
extern crate flate2;
#[macro_use]
extern crate lazy_static;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;

// For databases:
extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_infer_schema;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate r2d2_redis;
extern crate redis;
extern crate uuid;

mod db;
mod compress;
pub mod api;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket::http::ContentType;
use rocket_contrib::Template;

use compress::*;

/// Homepage.
#[get("/")]
pub fn homepage() -> CompressedTemplate {
    /// Context structure for homepage.
    #[derive(Debug, Serialize)]
    struct HomepageContext {
        /// Title of the page, owned, so that it's not hardcoded and can vary.
        title: String,
        /// The short representation of the language.
        ///
        /// `"en"` for English, `"es"` for Spanish, and so on. Owned, so that it can be changed for
        /// each request.
        lang_short: String,
        /// CSS style for the page.
        ///
        /// Note that you should add anything not required on page load as a separate CSS linked
        /// file, to keep the first load to a minimum.
        css: &'static str,
        /// JavaScript script for the page.
        ///
        /// Note that you should add anything not required on page load as a separate JavaScript
        /// linked file, to keep the first load to a minimum.
        script: &'static str,
    }

    let context = HomepageContext {
        title: "Homepage".to_owned(),
        lang_short: "en".to_owned(),
        css: include_str!("../static/css/_compiled/homepage.css"),
        script: include_str!("../static/js/_compiled/homepage.min.js"),
    };
    CompressedTemplate::new(Template::render("homepage", &context))
}

/// Image.
#[get("/img/<file..>")]
pub fn image(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/img").join(file)).ok()
}

/// Main favicon.
#[get("/favicon.ico")]
pub fn favicon() -> Option<NamedFile> {
    NamedFile::open("static/favicon.ico").ok()
}

/// Android app configuration.
#[get("/fav/browserconfig.xml")]
pub fn android_config() -> Option<CompressedFile> {
    CompressedFile::new("static/fav/browserconfig.xml", ContentType::XML).ok()
}

/// Windows app configuration.
#[get("/fav/manifest.json")]
pub fn windows_config() -> Option<CompressedFile> {
    CompressedFile::new("static/fav/manifest.json", ContentType::JSON).ok()
}

/// Favicon file.
#[get("/fav/<file..>", rank = 2)]
pub fn favicons(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/fav").join(file)).ok()
}

/// CSS file.
#[get("/css/<file..>")]
pub fn css(file: PathBuf) -> Option<CompressedFile> {
    CompressedFile::new(
        Path::new("static/css/_compiled").join(file),
        ContentType::CSS,
    ).ok()
}

/// Gets a javascript file.
#[get("/js/<file..>")]
pub fn js(file: PathBuf) -> Option<CompressedFile> {
    CompressedFile::new(
        Path::new("static/js/_compiled").join(file),
        ContentType::JavaScript,
    ).ok()
}

/// JavaScript source maps.
#[cfg(feature = "source_maps")]
#[get("/js-map/<file..>")]
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn js_source_maps(file: PathBuf) -> Option<CompressedFile> {
    let extension = file.extension().map(|e| e.to_string_lossy().into_owned());

    if extension == Some("map".to_owned()) {
        CompressedFile::new(Path::new("static/js/_map").join(&file), ContentType::JSON).ok()
    } else {
        None
    }
}

/// CSS ource maps and sources.
#[cfg(feature = "source_maps")]
#[get("/<file..>", rank = 100)]
pub fn source_maps(file: PathBuf) -> Option<CompressedFile> {
    let extension = file.extension().map(|e| e.to_string_lossy().into_owned());
    if extension == Some("map".to_owned()) {
        CompressedFile::new(Path::new("static/css/_map").join(file), ContentType::JSON).ok()
    } else if extension == Some("scss".to_owned()) {
        CompressedFile::new(
            Path::new("static/css").join(file),
            ContentType::new("text", "x-scss"),
        ).ok()
    } else if extension == Some("sass".to_owned()) {
        CompressedFile::new(
            Path::new("static/css").join(file),
            ContentType::new("text", "x-sass"),
        ).ok()
    } else if extension == Some("js".to_owned()) {
        CompressedFile::new(file, ContentType::JavaScript).ok()
    } else {
        None
    }
}
