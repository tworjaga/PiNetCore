import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import { useQuery } from '@tanstack/react-query';

interface Connection {
  ip: string;
  port: number;
  timestamp: string;
}

const mockData = [
  { time: '00:00', packets: 12 },
  { time: '00:05', packets: 19 },
  // ... more points
];

export function TrafficGraph() {
  const { data } = useQuery({
    queryKey: ['traffic'],
    queryFn: () => Promise.resolve(mockData),
  });

  return (
    <div className="h-96">
      <ResponsiveContainer>
        <LineChart data={data || []}>
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis dataKey="time" />
          <YAxis />
          <Tooltip />
          <Line type="monotone" dataKey="packets" stroke="#8884d8" />
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
}

