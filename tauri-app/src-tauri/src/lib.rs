use tauri::{WebviewWindowBuilder, WebviewUrl};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();

            WebviewWindowBuilder::new(&handle, "main", WebviewUrl::App("".into()))
                .initialization_script(r#"
                    window.__TAURI_INTERNALS__.invoke("plugin:opener|open_url", {url: "https://google.com"} );
                "#) 
                .initialization_script(r#"
                    setTimeout(() => {
                        window.__TAURI_INTERNALS__.invoke("plugin:opener|open_url", {url: "https://bing.com"} );
                    }, 100);
                "#)
                .build()
                .expect("error building webview window");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
