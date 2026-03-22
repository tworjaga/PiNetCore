// Zustand store stub
import { create } from 'zustand';

interface State {
  connections: any[];
  setConnections: (conns: any[]) => void;
}

export const useStore = create<State>((set) => ({
  connections: [],
  setConnections: (conns) => set({ connections: conns }),
}));
</xai:function_call name="create_file">

<xai:function_call name="create_file">
<parameter name="absolute_path">d:/pythonprojects/rpi-net-appliance/dashboard/src/types/index.ts
