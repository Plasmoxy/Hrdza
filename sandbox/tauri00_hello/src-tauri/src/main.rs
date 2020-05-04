#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

fn main() {
  println!("MAIN");
  
  
  
  tauri::AppBuilder::new()
    .setup(|webview, _source| {
      tauri::event::listen(String::from("test"), move |msg| {
        println!("got js-event with message '{:?}'", msg);
      });
    })
    .invoke_handler(|_webview, arg| {
      println!("COMAND REC");
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => {
          Err(e.to_string())
        }
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            MyCustomCommand { argument } => {
              //  your command code
              println!("{}", argument);
            }
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
    
  
}
