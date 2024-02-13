use leptos::*;

#[server(endpoint = "hello_world")]
pub async fn hello_world_server() -> Result<String, ServerFnError> {
	Ok("Hey.".to_string())
}
