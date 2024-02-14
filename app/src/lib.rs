use leptos::*;
use leptos_meta::{provide_meta_context, Stylesheet};
use server::*;

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
		<Stylesheet id="leptos" href="/pkg/leptos_tauri.css"/>
		<button
		class = "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded "
		on:click=move |_| {
			action.dispatch(HelloWorldServer{});
		}
		>"Hello world."</button>
		{
			move || vals.get()
		}
	}
}
