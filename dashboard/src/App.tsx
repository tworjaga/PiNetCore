import { BrowserRouter as Router, Routes, Route, Link, useEffect } from 'react-router-dom'
import { Layout } from './components/layout/Layout'
import { ConnectionsWidget } from './components/widgets/ConnectionsWidget'
import { TrafficGraph } from './components/widgets/TrafficGraph'
import { PluginManager } from './components/PluginManager'
import { wsService } from './services/websocket'

function App() {
  useEffect(() => {
    wsService.connect('ws://localhost:8080/api/ws')
    wsService.onMessage((data) => console.log('Live data:', data))
    const interval = setInterval(() => wsService.sendPing(), 5000)
    return () => clearInterval(interval)
  }, [])

  return (
    <Router>
      <Layout>
        <Routes>
          <Route path="/" element={<DashboardHome />} />
          <Route path="/connections" element={<ConnectionsWidget />} />
          <Route path="/traffic" element={<TrafficGraph />} />
          <Route path="/plugins" element={<PluginManager />} />
          <Route path="/firewall" element={<div>Firewall Panel</div>} />
        </Routes>
      </Layout>
    </Router>
  )
}

function DashboardHome() {
  return (
    <div className="p-8">
      <h1 className="text-3xl font-bold">PiNetCore Dashboard</h1>
      <nav className="mt-8 space-x-4">
        <Link to="/connections" className="bg-blue-500 text-white px-4 py-2 rounded">Connections</Link>
        <Link to="/traffic" className="bg-purple-500 text-white px-4 py-2 rounded">Traffic Graph</Link>
        <Link to="/plugins" className="bg-green-500 text-white px-4 py-2 rounded">Plugins</Link>
        <Link to="/firewall" className="bg-orange-500 text-white px-4 py-2 rounded">Firewall</Link>
      </nav>
    </div>
  )
}

export default App

function DashboardHome() {
  return (
    <div className="p-8">
      <h1 className="text-3xl font-bold">PiNetCore Dashboard</h1>
      <p className="mt-4 text-gray-600">Network appliance control center</p>
      <nav className="mt-8 space-x-4">
        <Link to="/connections" className="bg-blue-500 text-white px-4 py-2 rounded">Connections</Link>
        <Link to="/firewall" className="bg-orange-500 text-white px-4 py-2 rounded">Firewall</Link>
        <Link to="/plugins" className="bg-green-500 text-white px-4 py-2 rounded">Plugins</Link>
      </nav>
    </div>
  )
}

export default App

