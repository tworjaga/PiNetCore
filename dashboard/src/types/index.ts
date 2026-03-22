export interface Connection {
  id: number;
  ip: string;
  port: number;
  device?: string;
  timestamp: string;
}

export interface Plugin {
  name: string;
  enabled: boolean;
}

