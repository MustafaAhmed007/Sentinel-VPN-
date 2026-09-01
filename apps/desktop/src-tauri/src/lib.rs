use std::{fs, net::TcpStream, time::Duration};
use sentinel_ipc::{read_frame, write_frame, Command, Envelope, Event, Response};

fn ipc_token() -> Result<String,String> {
    if let Ok(v)=std::env::var("SENTINEL_IPC_TOKEN"){return Ok(v)}
    let path=std::env::var("PROGRAMDATA").unwrap_or_else(|_|"C:\\ProgramData".into())+"\\SentinelVPN\\ipc.token";
    fs::read_to_string(path).map(|s|s.trim().to_owned()).map_err(|_|"Sentinel service token is unavailable".into())
}
fn request(command:Command)->Result<Event,String>{let token=ipc_token()?;let addr=std::env::var("SENTINEL_IPC_LISTEN").unwrap_or_else(|_|"127.0.0.1:39421".into());let mut stream=TcpStream::connect_timeout(&addr.parse().map_err(|_|"invalid IPC address")?,Duration::from_secs(2)).map_err(|e|e.to_string())?;stream.set_read_timeout(Some(Duration::from_secs(5))).ok();let req=Envelope{request_id:format!("{}",std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|_|"clock error")?.as_nanos()),auth_token:token,command};write_frame(&mut stream,&req).map_err(|e|e.to_string())?;let response:Response=read_frame(&mut stream).map_err(|e|e.to_string())?;Ok(response.event)}

#[derive(Clone,serde::Serialize)] struct TrayStatus{connected:bool,state:String}
#[tauri::command] fn service_status()->Result<TrayStatus,String>{match request(Command::GetStatus)?{Event::Status{state}=>Ok(TrayStatus{connected:state=="Connected",state}),Event::Error{message,..}=>Err(message),other=>Err(format!("unexpected IPC response: {other:?}"))}}
#[tauri::command] fn service_disconnect(interface_name:String)->Result<TrayStatus,String>{match request(Command::Disconnect{interface_name})?{Event::Status{state}=>Ok(TrayStatus{connected:false,state}),Event::Error{message,..}=>Err(message),other=>Err(format!("unexpected IPC response: {other:?}"))}}
#[tauri::command] fn service_connect(profile:serde_json::Value,dns:serde_json::Value,endpoint:String)->Result<TrayStatus,String>{match request(Command::Connect{profile,dns,endpoint})?{Event::Status{state}=>Ok(TrayStatus{connected:state=="Connected",state}),Event::Error{message,..}=>Err(message),other=>Err(format!("unexpected IPC response: {other:?}"))}}

pub fn run(){tauri::Builder::default().invoke_handler(tauri::generate_handler![service_status,service_connect,service_disconnect]).setup(|app|{if let Some(window)=app.get_webview_window("main"){let _=window.emit("sentinel-ready",true);}Ok(())}).run(tauri::generate_context!()).expect("error while running Sentinel-VPN Ω");}
