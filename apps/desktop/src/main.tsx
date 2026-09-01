import React from 'react';
import { createRoot } from 'react-dom/client';
import './styles.css';

type Status = 'Disconnected' | 'Protected';

function App() {
  const [status, setStatus] = React.useState<Status>('Disconnected');

  return (
    <main className="app-shell">
      <header className="topbar">
        <div>
          <div className="brand">SENTINEL <span>VPN Ω</span></div>
          <div className="subtitle">Private network, enforced by policy.</div>
        </div>
        <div className={`status-pill ${status === 'Protected' ? 'ok' : ''}`}>
          <span className="status-dot" /> {status}
        </div>
      </header>

      <section className="hero-card">
        <div className="shield">{status === 'Protected' ? '✓' : '◎'}</div>
        <div className="state">{status === 'Protected' ? 'Protected' : 'Ready'}</div>
        <div className="endpoint">{status === 'Protected' ? 'Singapore · WireGuard' : 'No active tunnel'}</div>
        <button
          className="connect"
          onClick={() => setStatus(status === 'Protected' ? 'Disconnected' : 'Protected')}
        >
          {status === 'Protected' ? 'Disconnect' : 'Connect'}
        </button>
      </section>

      <section className="metrics">
        <article><strong>{status === 'Protected' ? '184' : '—'}</strong><span>Mbps down</span></article>
        <article><strong>{status === 'Protected' ? '31' : '—'}</strong><span>Mbps up</span></article>
        <article><strong>{status === 'Protected' ? '24' : '—'}</strong><span>ms latency</span></article>
      </section>

      <section className="health-grid">
        <Health name="Firewall" value={status === 'Protected' ? 'LOCKED' : 'STANDBY'} />
        <Health name="DNS" value={status === 'Protected' ? 'SAFE' : 'READY'} />
        <Health name="IPv6" value={status === 'Protected' ? 'SAFE' : 'READY'} />
        <Health name="P2P" value={status === 'Protected' ? 'READY' : 'OFF'} />
      </section>

      <footer>
        <span>Fail-closed security policy</span>
        <span>Sentinel-VPN Ω v0.1.0</span>
      </footer>
    </main>
  );
}

function Health({ name, value }: { name: string; value: string }) {
  return <div className="health"><span>{name}</span><strong>{value}</strong></div>;
}

createRoot(document.getElementById('root')!).render(
  <React.StrictMode><App /></React.StrictMode>,
);
