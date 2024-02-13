# Tailwind-Leptos-Tauri Template

Simple template to use csr and ssr leptos with tauri for ios/android/windows/macos/linux and web dev

Just clone the repo and start your project with the template.
To test everything is working, run `cargo tauri dev` and 'cargo leptos watch' to see the app in the browser and app.

Thanks to the leptos and tauri teams for the amazing work.
Also all credit to [sjud](https://github.com/sjud/leptos_tauri_from_scratch/tree/main) and [Krzysztof](https://gitlab.com/cristofa/tauri-leptos-template) for their examples, as they were the base for this template.

This project differs as it:

- Adds tailwind to sjud's example
- Adds SSR to Krzysztof
- Separates the server from the frontend crate
- Separated the app module from the frontend crate

Mobile dev:
iOS requires a mac and XCode
Android is setup to use gradle 8.5 and jvm 21 but by default tauri uses 8.0
You can changge it in the distributionUrl and for compatibility [check](https://docs.gradle.org/current/userguide/compatibility.html)

CORS policy disclaimer: couldnt be bothered to fix simulators connection
to machine so just put any, but delete that line and customize as you please
