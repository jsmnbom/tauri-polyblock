#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
use strum::VariantNames;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use serde_json::{Value};

fn main() {
  tauri::AppBuilder::new()
    // .invoke_handler(|_webview, arg| {
    //   println!("{:?}", arg);
    //   use cmd::Cmd::*;
    //   match serde_json::from_str(arg) {
    //     Err(_) => {}
    //     Ok(command) => {
    //       match command {
    //         // definitions for your custom commands from Cmd here
    //         MyCustomCommand { argument } => {
    //           //  your command code
    //           println!("{}", argument);
    //         }
    //       }
    //     }
    //   }
    // })
.setup(|_webview| {
  //let handle2 = _webview.handle();
  use cmd::Cmd;
  
  for variant in Cmd::VARIANTS {
    tauri::event::listen((*variant).to_string(), move |msg| {
      let mut data: Value = serde_json::from_str(&msg).unwrap();
      let map = data.as_object_mut().unwrap();
      map.insert(String::from("cmd"), Value::String((*variant).to_string()));
      let cmd_enum: Cmd = serde_json::from_value(data).unwrap();

      use cmd::Cmd::*;
      match cmd_enum {
        MyCustomCommand { msg } => {
          println!("the message is: {}", msg);
        }
      }
    });
  }
})
    .build()
    .run();
}
