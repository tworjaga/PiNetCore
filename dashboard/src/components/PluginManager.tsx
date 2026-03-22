interface Plugin {
  name: string;
  enabled: boolean;
}

const plugins: Plugin[] = [
  { name: 'WireGuard VPN', enabled: true },
  { name: 'Pi-hole Adblock', enabled: true },
  { name: 'Suricata IDS', enabled: false },
];

export function PluginManager() {
  const togglePlugin = (name: string) => {
    console.log(`Toggle ${name}`);
    // API call to /api/plugins/{name}/toggle
  };

  return (
    <div className="p-6">
      <h2 className="text-2xl font-bold mb-4">Plugins</h2>
      <div className="space-y-2">
        {plugins.map((plugin) => (
          <div key={plugin.name} className="flex justify-between items-center p-4 bg-white rounded shadow">
            <span>{plugin.name}</span>
            <button 
              onClick={() => togglePlugin(plugin.name)}
              className={`px-4 py-2 rounded ${plugin.enabled ? 'bg-green-500' : 'bg-gray-500'} text-white`}
            >
              {plugin.enabled ? 'ON' : 'OFF'}
            </button>
          </div>
        ))}
      </div>
    </div>
  );
}

