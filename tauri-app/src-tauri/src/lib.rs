use tauri::{WebviewWindowBuilder, WebviewUrl};
use log::{SetLoggerError, LevelFilter};
#[macro_use] extern crate log;
extern crate simplelog;
use simplelog::*;

#[tauri::command]
fn greet(name: &str) {
    log::info!("Hello from {}", name);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]

pub fn run() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
       ]
    ).unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let handle = app.handle().clone();

            WebviewWindowBuilder::new(&handle, "main", WebviewUrl::App("".into()))
                .initialization_script(r#"
                    window.__TAURI_INTERNALS__.invoke("greet", {name: "script 1"});
                "#) 
                .initialization_script(r#"
                    setTimeout(() => {
                        window.__TAURI_INTERNALS__.invoke("greet", {name: "script 2"});
                    }, 100);
                "#)
                .build()
                .expect("error building webview window");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
