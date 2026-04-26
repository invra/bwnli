use maud::{DOCTYPE, html};
use std::fs;

fn main() {
    let markup = html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Bitwig NLI" }
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css";
                link data-trunk rel="rust" data-bin="bwnli-website";
            }
            body { }
        }
    };

    fs::write("index.html", markup.into_string()).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
