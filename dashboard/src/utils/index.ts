export const formatBytes = (bytes: number) => {
  const units = ['B', 'KB', 'MB'];
  let div = 1;
  let unit = 'B';
  for (let i = 0; i < units.length; i++) {
    if (bytes / div < 1024) {
      unit = units[i];
      break;
    }
    div *= 1024;
  }
  return (bytes / div).toFixed(1) + unit;
};

