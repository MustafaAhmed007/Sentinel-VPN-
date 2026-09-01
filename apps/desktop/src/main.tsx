import React from 'react';
import { createRoot } from 'react-dom/client';
import { invoke } from '@tauri-apps/api/core';
import './styles.css';

type Status={connected:boolean;state:string};
type Profile={interface_name:string;private_key:string;address:string[];dns:string[];peer:{public_key:string;endpoint:string;allowed_ips:string[];persistent_keepalive:number|null}};
type Dns={servers:string[];block_plain_dns:boolean;block_plain_doh:boolean;prevent_fallback:boolean};
const defaultProfile:Profile={interface_name:'Sentinel',private_key:'',address:['10.66.66.2/32'],dns:['10.66.66.1'],peer:{public_key:'',endpoint:'',allowed_ips:['0.0.0.0/0','::/0'],persistent_keepalive:25}};
const defaultDns:Dns={servers:['10.66.66.1'],block_plain_dns:true,block_plain_doh:true,prevent_fallback:true};
function App(){
 const [status,setStatus]=React.useState<Status>({connected:false,state:'Disconnected'});const[profile,setProfile]=React.useState(defaultProfile);const[dns,setDns]=React.useState(defaultDns);const[error,setError]=React.useState('');const[showConfig,setShowConfig]=React.useState(false);
 const refresh=async()=>{try{setStatus(await invoke<Status>('service_status'))}catch(e){setError(String(e))}};React.useEffect(()=>{refresh()},[]);
 const connect=async()=>{setError('');if(!profile.private_key||!profile.peer.public_key||!profile.peer.endpoint){setShowConfig(true);setError('Add the WireGuard private key, server public key and endpoint first.');return}try{setStatus(await invoke<Status>('service_connect',{profile,dns,endpoint:profile.peer.endpoint}))}catch(e){setError(String(e))}};
 const disconnect=async()=>{setError('');try{setStatus(await invoke<Status>('service_disconnect',{interfaceName:profile.interface_name}))}catch(e){setError(String(e))}};
 return <main className="app-shell"><header className="topbar"><div><div className="brand">SENTINEL <span>VPN Ω</span></div><div className="subtitle">Private network, enforced by policy.</div></div><div className={`status-pill ${status.connected?'ok':''}`}><span className="status-dot"/> {status.connected?'Protected':status.state}</div></header>
 <section className="hero-card"><div className="shield">{status.connected?'✓':'◎'}</div><div className="state">{status.connected?'Protected':'Ready'}</div><div className="endpoint">{status.connected?profile.peer.endpoint:'No active tunnel'}</div><button className="connect" onClick={status.connected?disconnect:connect}>{status.connected?'Disconnect':'Connect'}</button></section>
 <section className="health-grid"><Health name="Firewall" value={status.connected?'LOCKED':'STANDBY'}/><Health name="DNS" value={status.connected?'SAFE':'READY'}/><Health name="IPv6" value={status.connected?'SAFE':'READY'}/><Health name="P2P" value={status.connected?'READY':'OFF'}/></section>
 <section className="actions"><button onClick={()=>setShowConfig(v=>!v)}>{showConfig?'Hide configuration':'Configure WireGuard'}</button><button onClick={refresh}>Refresh status</button></section>
 {showConfig&&<section className="config"><label>Interface<input value={profile.interface_name} onChange={e=>setProfile({...profile,interface_name:e.target.value})}/></label><label>VPN endpoint<input placeholder="203.0.113.10:51820" value={profile.peer.endpoint} onChange={e=>setProfile({...profile,peer:{...profile.peer,endpoint:e.target.value}})}/></label><label>Server public key<input value={profile.peer.public_key} onChange={e=>setProfile({...profile,peer:{...profile.peer,public_key:e.target.value}})}/></label><label>Private key<input type="password" autoComplete="off" value={profile.private_key} onChange={e=>setProfile({...profile,private_key:e.target.value})}/></label><label>VPN address<input value={profile.address[0]} onChange={e=>setProfile({...profile,address:[e.target.value]})}/></label><label>DNS server<input value={dns.servers[0]} onChange={e=>{setDns({...dns,servers:[e.target.value]});setProfile({...profile,dns:[e.target.value]})}}/></label><p className="hint">Private keys remain in application memory for this session and are never written to logs.</p></section>}
 {error&&<div className="error">{error}</div>}<footer><span>Fail-closed security policy</span><span>Sentinel-VPN Ω v0.1.0</span></footer></main>;
}
function Health({name,value}:{name:string;value:string}){return <div className="health"><span>{name}</span><strong>{value}</strong></div>}
createRoot(document.getElementById('root')!).render(<React.StrictMode><App/></React.StrictMode>);
