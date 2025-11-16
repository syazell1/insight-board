import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AuthProvider } from "../context/AuthContext";
import { type ReactNode } from "react";

const queryClient = new QueryClient();

const QueryProvider = ({ children }: { children: ReactNode }) => {
  return (
    <AuthProvider>
      <QueryClientProvider client={queryClient}>
        {children}
      </QueryClientProvider>
    </AuthProvider>
  );
}

export default QueryProvider;
