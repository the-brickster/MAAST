// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::format;
use std::process::Output;
use std::sync::Arc;
use std::task::Context;



use tauri::Manager;
use taurpc::Router;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tracing::info;
use tracing_subscriber;
use rand::Rng;

mod components;

#[tauri::command]
fn test_invoke(name: &str) -> String {
  print!("GOT THE FOLLOWING: {}",name);
  format!("Hello, {}!", name)
}
#[tauri::command]
async fn js2rs(payload:String,state:tauri::State<'_,AsyncProcInputTx>) -> Result<(),String>{
  info!(?payload,"js2rs");
  let async_proc_input_tx = state.inner.lock().await;
  async_proc_input_tx.send(payload).await.map_err(|e| e.to_string())
}

fn rs2js<R: tauri::Runtime>(message:String,manager: &impl Manager<R>){
  let mut rng = rand::thread_rng();
  info!(?message,"rs2js");
  manager.emit_all("rs2js", format!("rs: {}, {}",message,rng.gen_range(0..100))).unwrap();
}

async fn async_process_model(mut input_rx: mpsc::Receiver<String>,output_tx: mpsc::Sender<String>)->Result<(),Box<dyn std::error::Error+Send+Sync>>{
  while let Some(input) = input_rx.recv().await  {
      let output = input;
      output_tx.send(output).await?;
  }
  Ok(())
}

fn test_tauri_rspc(_:Context,test:String) -> String{
  format!("Testing tauri rspc {}",test)
}

struct AsyncProcInputTx {
  inner: Mutex<mpsc::Sender<String>>,
}


#[tokio::main]
async fn main() {
  
  tracing_subscriber::fmt::init();

  let(async_proc_input_tx, async_proc_input_rx) = mpsc::channel(1);
  let (async_proc_output_tx, mut async_proc_output_rx) = mpsc::channel(1);
  let router = components::gen_router();

  
  tauri::Builder::default()
  .manage(AsyncProcInputTx{
    inner: Mutex::new(async_proc_input_tx),
  })
  // .invoke_handler(tauri::generate_handler![test_invoke,js2rs])
  .invoke_handler(router.into_handler())
  .setup(|app|{
    app.get_window("main").unwrap().open_devtools();
    tauri::async_runtime::spawn(async move{
      async_process_model(async_proc_input_rx, async_proc_output_tx).await
    });
    let app_handle = app.handle();
    tauri::async_runtime::spawn(async move{
      loop {
          if let Some(output) = async_proc_output_rx.recv().await{
            rs2js(output, &app_handle);
          }
      }
    });
    Ok(())
  })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
