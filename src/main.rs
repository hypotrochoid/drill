#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use {
    clap::Parser,
    drill::units::app::{AppWrapper, SeekerApp, SeekerArgs},
    std::sync::Arc,
};

fn main() {
    let args = SeekerArgs::parse();

    let app = AppWrapper {
        app: Arc::new(SeekerApp::new(args).expect("failed to start app")),
    };

    start(app)
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn start(app: AppWrapper) {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    app.run_native();
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn start(app: AppWrapper) {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    app.run_webgui();
}
