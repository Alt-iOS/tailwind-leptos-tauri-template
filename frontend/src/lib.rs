cfg_if::cfg_if! {
	if #[cfg(feature = "hydrate")] {
		use app::App;
		use wasm_bindgen::prelude::wasm_bindgen;

		#[wasm_bindgen]
		pub fn hydrate() {
				_ = console_log::init_with_level(log::Level::Debug);
			#[cfg(debug_assertions)]
			console_error_panic_hook::set_once();
			leptos::mount_to_body(App);
		}
	}
}
