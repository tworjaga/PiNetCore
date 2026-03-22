interface ButtonProps {
  children: React.ReactNode;
  variant?: 'primary' | 'secondary';
}

export function Button({ children, variant = 'primary' }: ButtonProps) {
  const base = 'px-4 py-2 rounded font-medium';
  const variants = {
    primary: 'bg-blue-500 text-white hover:bg-blue-600',
    secondary: 'bg-gray-200 text-gray-800 hover:bg-gray-300',
  };

  return (
    <button className={`${base} ${variants[variant]}`}>
      {children}
    </button>
  );
}

