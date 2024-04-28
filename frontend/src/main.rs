#[cfg(not(feature = "ssr"))]
pub fn main() {
	use app::App;
	_ = console_log::init_with_level(log::Level::Debug);
	server_fn::client::set_server_url("http://127.0.0.1:8000");
	leptos::mount_to_body(App);
}
