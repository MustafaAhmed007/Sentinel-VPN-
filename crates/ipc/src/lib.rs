use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag="type",content="payload")]
pub enum Command { GetStatus, Connect { profile:serde_json::Value,dns:serde_json::Value,endpoint:String }, Disconnect { interface_name:String }, Reconnect, RunDiagnostics, ListProfiles, GetProfile { profile_id:String }, SaveProfile { profile:serde_json::Value }, DeleteProfile { profile_id:String }, AllocateP2pPort, RevokeP2pPort { port:u16 } }
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag="type",content="payload")]
pub enum Event { StateChanged{state:String}, DiagnosticResult{report:serde_json::Value}, P2pPortAllocated{port:u16}, Status{state:String}, Error{code:String,message:String} }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope { pub request_id:String, pub auth_token:String, pub command:Command }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response { pub request_id:String, pub event:Event }
pub fn write_frame<W:Write,T:Serialize>(w:&mut W,value:&T)->io::Result<()> { let bytes=serde_json::to_vec(value).map_err(io::Error::other)?; let len=u32::try_from(bytes.len()).map_err(|_|io::Error::new(io::ErrorKind::InvalidData,"IPC frame too large"))?; w.write_all(&len.to_be_bytes())?;w.write_all(&bytes)?;w.flush() }
pub fn read_frame<R:Read,T:for<'de>Deserialize<'de>>(r:&mut R)->io::Result<T> { let mut h=[0u8;4];r.read_exact(&mut h)?;let len=u32::from_be_bytes(h) as usize;if len>1024*1024{return Err(io::Error::new(io::ErrorKind::InvalidData,"IPC frame exceeds 1 MiB"))};let mut b=vec![0u8;len];r.read_exact(&mut b)?;serde_json::from_slice(&b).map_err(io::Error::other) }
#[cfg(test)] mod tests {use super::*;#[test]fn round_trip(){let e=Envelope{request_id:"r1".into(),auth_token:"t".into(),command:Command::GetStatus};let mut b=Vec::new();write_frame(&mut b,&e).unwrap();let got:Envelope=read_frame(&mut b.as_slice()).unwrap();assert_eq!(got.auth_token,"t");}}
