#[cfg(target_arch = "wasm32")]
fn main() {
    aionfm_ui::web::run();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!(
        "aionfm-ui is a Rust/WASM app. Use `trunk serve` with the wasm32-unknown-unknown target."
    );
}
