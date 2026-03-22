export class WebSocketService {
  private ws: WebSocket | null = null;
  private callbacks: ((data: any) => void)[] = [];

  connect(url: string) {
    this.ws = new WebSocket(url);
    this.ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      this.callbacks.forEach(cb => cb(data));
    };
    this.ws.onopen = () => console.log('WS connected');
  }

  onMessage(callback: (data: any) => void) {
    this.callbacks.push(callback);
  }

  sendPing() {
    if (this.ws) this.ws.send('ping');
  }
}

export const wsService = new WebSocketService();

