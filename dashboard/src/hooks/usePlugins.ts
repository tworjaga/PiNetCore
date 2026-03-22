import { useQuery } from '@tanstack/react-query';

export function usePlugins() {
  return useQuery({
    queryKey: ['plugins'],
    queryFn: () => fetch('/api/plugins').then(res => res.json()),
  });
}

