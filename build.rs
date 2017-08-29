//! Build script.

#[macro_use]
extern crate error_chain;

use std::process::Command;
use std::fs::{read_dir, create_dir_all, rename};
use std::path::{Path, PathBuf};

use error::*;

fn main() {
    // Minify CSS:
    minify_css("static/css").expect("there was an error minifying the CSS");

    // Minify JavaScript:
    minify_js("static/js").expect("there was an error minifying the JavaScript");
}

/// Minifies a SCSS folder.
fn minify_css<P: AsRef<Path>>(folder_path: P) -> Result<()> {
    for file in read_dir(&folder_path)? {
        let file = file?;
        let path = file.path();
        let file_type = file.file_type()?;
        if file_type.is_file() &&
            path.extension().map(|e| e.to_string_lossy().into_owned()) == Some("scss".to_owned())
        {

            let mut new_file =
                PathBuf::from("static/css/_compiled").join(path.strip_prefix("static/css")?);
            new_file.set_extension("css");

            let mut dir = new_file.clone();
            dir.pop();
            if !dir.exists() {
                create_dir_all(dir)?;
            }

            let mut command = Command::new("sass");
            command
                .arg("-t")
                .arg("compressed")
                .arg("--unix-newlines")
                .arg(file.path())
                .arg(&new_file);
            if cfg!(feature = "source_maps") {
                command.arg("--sourcemap=auto");
            } else {
                command.arg("--sourcemap=none");
            }

            let output = command.output()?;
            if !output.status.success() {
                let stderr = String::from_utf8(output.stderr)?;
                panic!("Error minifying CSS: {}", stderr);
            }

            if !new_file.exists() {
                bail!(
                    "Seems that the `{}` CSS file was not generated correctly",
                    new_file.display()
                );
            }
            // Move source maps to a directory where they can be shown.
            if cfg!(feature = "source_maps") {
                let mut map_file = PathBuf::from(&new_file);
                map_file.set_extension("css.map");
                if !map_file.exists() {
                    bail!(
                        "Seems that the `{}` CSS map file was not generated correctly",
                        map_file.display()
                    );
                } else {
                    let new_file = Path::new("static/css/_map").join(map_file.file_name().unwrap());
                    rename(map_file, new_file)?;
                }
            }
        } else if file_type.is_dir() &&
                   !path.file_name().unwrap().to_str().unwrap().starts_with('_')
        {
            minify_css(path)?;
        }
    }
    Ok(())
}

/// Minifies a JavaScript folder.
fn minify_js<P: AsRef<Path>>(folder_path: P) -> Result<()> {
    for file in read_dir(&folder_path)? {
        let file = file?;
        let path = file.path();
        let file_type = file.file_type()?;
        if file_type.is_file() &&
            path.extension().map(|e| e.to_string_lossy().into_owned()) == Some("js".to_owned())
        {

            let relative_path = path.strip_prefix("static/js")?;
            let mut new_file = PathBuf::from("static/js/_compiled").join(&relative_path);
            new_file.set_extension("min.js");

            let mut dir = new_file.clone();
            dir.pop();
            if !dir.exists() {
                create_dir_all(dir)?;
            }

            let mut command = Command::new("uglifyjs");
            command
                .arg(file.path())
                .arg("--screw-ie8")
                .arg("-m")
                .arg("-c")
                .arg("--mangle-props")
                .arg("-o")
                .arg(&new_file);
            if cfg!(feature = "source_maps") {
                let mut map_file = PathBuf::from("static/js/_map").join(&relative_path);
                map_file.set_extension("min.js.map");

                let url = Path::new("/js-map").join(map_file.strip_prefix("static/js/_map")?);

                command.arg("--source-map").arg(format!(
                    "root='/',url='{}',filename='{}'",
                    url.display(),
                    map_file.display()
                ));
            }

            let output = command.output()?;
            if !output.status.success() {
                let stderr = String::from_utf8(output.stderr)?;
                panic!("Error minifying JavaScript: {}", stderr);
            }

            if !new_file.exists() {
                panic!(
                    "Seems that the `{}` JavaScript file was not generated correctly",
                    new_file.display()
                );
            }
        } else if file_type.is_dir() &&
                   !path.file_name().unwrap().to_str().unwrap().starts_with('_') &&
                   path.file_name().unwrap().to_str().unwrap() != "compiled"
        {
            minify_js(path)?;
        }
    }
    Ok(())
}

/// Error module.
#[allow(unused_doc_comment)]
mod error {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            StripPrefix(::std::path::StripPrefixError);
            FromUtf8(::std::string::FromUtf8Error);
        }
    }
}
