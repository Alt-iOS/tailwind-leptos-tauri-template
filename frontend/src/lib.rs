use leptos::*;
use leptos_meta::{provide_meta_context, Stylesheet};
#[cfg(feature = "ssr")]
pub mod fallback;

#[server(endpoint = "hello_world")]
pub async fn hello_world_server() -> Result<String, ServerFnError> {
	Ok("Hey.".to_string())
}

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();
	let action = create_server_action::<HelloWorldServer>();
	let vals = create_rw_signal(String::new());
	create_effect(move |_| {
		if let Some(resp) = action.value().get() {
			match resp {
				Ok(val) => vals.set(val),
				Err(err) => vals.set(format!("{err:?}")),
			}
		}
	});
	view! {
				<Stylesheet id="leptos" href="/pkg/leptos_tauri_from_scratch.css"/>

		<button
		class = "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
		on:click=move |_| {
			action.dispatch(HelloWorldServer{});
		}
		>"Hello world."</button>
		{
			move || vals.get()
		}
	}
}

cfg_if::cfg_if! {
	if #[cfg(feature = "hydrate")] {
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
