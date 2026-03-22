const API_BASE = import.meta.env.VITE_API_URL || 'http://localhost:8080/api';

export async function getConnections() {
  const res = await fetch(`${API_BASE}/connections`);
  return res.json();
}

export async function togglePlugin(name: string, enabled: boolean) {
  const res = await fetch(`${API_BASE}/plugins/${name}`, {
    method: 'POST',
    body: JSON.stringify({ enabled }),
  });
  return res.json();
}

