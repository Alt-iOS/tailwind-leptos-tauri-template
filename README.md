# Tailwind-Leptos-Tauri Template

Simple template to use ssr leptos with tauri for ios/android/windows/macos/linux and web dev

To get started just use cargo generate with this repo.

```bash
cargo generate --git https://github.com/Alt-iOS/tailwind-leptos-tauri-template.git
```

To test everything is working, run `cargo tauri dev` and `cargo leptos watch` to see the app in the browser and app.

Thanks to the leptos and tauri teams for the amazing work.
Also all credit to [sjud](https://github.com/sjud/leptos_tauri_from_scratch/tree/main) and [Krzysztof](https://gitlab.com/cristofa/tauri-leptos-template) for their examples, as they were the base for this template.

This project differs as it:

- Adds tailwind to sjud's example
- Adds SSR to Krzysztof
- Separates the server from the frontend crate
- Separated the app module from the frontend crate
- Separated the backend logic from the rest

Structure:

- App: UI section
- frontend: launch point for the hydration/csr
- backend: server logic and server functions
- server: the launch point of the axum server
- src-tauri: tauri related settings, build script

Mobile dev:
`cargo tauri android init && cargo tauri ios init`

- iOS requires a mac and XCode
- Android requires JVM 17 or modifying the gradle version.
  You can changge it in the distributionUrl and for compatibility [check](https://docs.gradle.org/current/userguide/compatibility.html)
