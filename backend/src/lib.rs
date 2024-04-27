use leptos::*;

#[cfg(feature = "ssr")]
pub mod fallback;

#[server(endpoint = "hello_world")]
pub async fn hello_world_server() -> Result<String, ServerFnError> {
	Ok("Hey.".to_string())
}
