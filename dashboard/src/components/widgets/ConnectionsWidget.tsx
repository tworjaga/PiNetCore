import { useQuery } from '@tanstack/react-query'

async function fetchConnections() {
  const res = await fetch('/api/connections')
  if (!res.ok) throw new Error('Failed to fetch')
  return res.json()
}

export function ConnectionsWidget() {
  const { data, error, isLoading } = useQuery({
    queryKey: ['connections'],
    queryFn: fetchConnections,
  })

  if (isLoading) return <div>Loading...</div>
  if (error) return <div>Error: {(error as Error).message}</div>

  return (
    <div className="p-6">
      <h2 className="text-2xl font-bold mb-4">Recent Connections</h2>
      <pre>{JSON.stringify(data, null, 2)}</pre>
    </div>
  )
}

