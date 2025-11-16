import { createContext, type ReactNode, useContext, useState } from "react";

type TAuthContext = {
  accessToken: string,
  setAccessToken: (data: string) => void
}

const AuthContext = createContext<TAuthContext>({} as TAuthContext);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [accessToken, setAt] = useState("");

  const setAccessToken = (data: string) => {
    setAt(data)
  }

  return (
    <AuthContext.Provider value={{
      accessToken,
      setAccessToken
    }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => useContext(AuthContext);
