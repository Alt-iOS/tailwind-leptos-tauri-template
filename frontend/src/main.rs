#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
	use app::App;
	use axum::{
		body::Body,
		extract::{OriginalUri, State},
		http::Request,
		response::IntoResponse,
		routing::get,
		Router,
	};
	use leptos::{provide_context, LeptosOptions, *};
	use leptos_axum::LeptosRoutes;
	use server::fallback::file_and_error_handler;
	use tower_http::{
		cors::{Any, CorsLayer},
		trace::TraceLayer,
	};
	use tracing::info_span;
	use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

	#[derive(Clone, Debug, axum_macros::FromRef)]
	pub struct ServerState {
		pub options: LeptosOptions,
		pub routes: Vec<leptos_router::RouteListing>,
	}

	pub async fn server_fn_handler(
		State(state): State<ServerState>,
		request: Request<Body>,
	) -> impl IntoResponse {
		leptos_axum::handle_server_fns_with_context(
			move || {
				provide_context(state.clone());
			},
			request,
		)
		.await
		.into_response()
	}

	pub async fn leptos_routes_handler(
		State(state): State<ServerState>,
		req: Request<Body>,
	) -> axum::response::Response {
		let handler = leptos_axum::render_route_with_context(
			state.options.clone(),
			state.routes.clone(),
			move || {
				provide_context("...");
			},
			App,
		);
		handler(req).await.into_response()
	}

	let conf = get_configuration(Some("./Cargo.toml")).await.unwrap();

	let leptos_options = conf.leptos_options;
	let addr = leptos_options.site_addr;
	let routes = leptos_axum::generate_route_list(App);
	tracing_subscriber::registry()
		.with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
			// axum logs rejections from built-in extractors with the `axum::rejection`
			// target, at `TRACE` level. `axum::rejection=trace` enables showing those events
			"frontend=debug,tower_http=debug,axum::rejection=trace".into()
		}))
		.with(tracing_subscriber::fmt::layer())
		.init();

	let state = ServerState { options: leptos_options, routes: routes.clone() };

	let cors = CorsLayer::new()
		.allow_methods([axum::http::Method::GET, axum::http::Method::POST])
		.allow_origin("tauri://localhost".parse::<axum::http::HeaderValue>().unwrap())
		.allow_origin("http://127.0.0.1:8080".parse::<axum::http::HeaderValue>().unwrap())
		.allow_origin(Any)
		.allow_headers(vec![axum::http::header::CONTENT_TYPE]);

	let app = Router::new()
		.route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
		.layer(cors)
		.layer(TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
			let path = if let Some(path) = request.extensions().get::<OriginalUri>() {
				path.0.path().to_owned()
			} else {
				request.uri().path().to_owned()
			};

			info_span!(
			"http_request",
			method = ?request.method(),
			path,
			)
		}))
		.leptos_routes_with_handler(routes, get(leptos_routes_handler))
		.fallback(file_and_error_handler)
		.with_state(state);

	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	logging::log!("listening on http://{}", &addr);
	axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
	use app::App;
	_ = console_log::init_with_level(log::Level::Debug);
	server_fn::client::set_server_url("http://127.0.0.1:8000");
	leptos::mount_to_body(App);
}

/*
async fn main() {
	use axum::Router;
	use leptos::{logging::log, *};
	use leptos_axum::{generate_route_list, LeptosRoutes};
	use ssr_modes_axum::{app::*, fallback::file_and_error_handler};

	let conf = get_configuration(None).await.unwrap();
	let addr = conf.leptos_options.site_addr;
	let leptos_options = conf.leptos_options;
	// Generate the list of routes in your Leptos App
	let routes = generate_route_list(App);

	// Explicit server function registration is no longer required
	// on the main branch. On 0.3.0 and earlier, uncomment the lines
	// below to register the server functions.
	// _ = GetPost::register();
	// _ = ListPostMetadata::register();

	let app = Router::new()
		.leptos_routes(&leptos_options, routes, || view! { <App/> })
		.fallback(file_and_error_handler)
		.with_state(leptos_options);

	// run our app with hyper
	// `axum::Server` is a re-export of `hyper::Server`
	log!("listening on http://{}", &addr);
	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	axum::serve(listener, app.into_make_service())
		.await
		.unwrap();
}



 */
