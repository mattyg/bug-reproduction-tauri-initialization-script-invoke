use tauri::{WebviewWindowBuilder, WebviewUrl};

#[tauri::command]
fn greet(name: &str) -> String {
    println!("Hello from {}", name);
    format!("Hello from {}", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let handle = app.handle().clone();

            WebviewWindowBuilder::new(&handle, "main", WebviewUrl::App("".into()))
                .initialization_script(r#"
                    window.__TAURI_INTERNALS__.invoke("greet", {name: "script 1"}).then((res) => alert(res));
                "#)
                .build()
                .expect("error building webview window");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
