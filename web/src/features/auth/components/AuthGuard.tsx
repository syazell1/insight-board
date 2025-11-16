import { type ReactNode, useEffect } from "react";
import { useGetCurrentUser } from "../hooks/useGetCurrentUser";
import { currentUserStore } from "../stores/currentUserStore";

const AuthGuard = ({ children }: { children: ReactNode }) => {
  const { isPending, data, isSuccess } = useGetCurrentUser();
  const { setCurrentUser } = currentUserStore()

  useEffect(() => {
    (async () => {
      if (isSuccess) {
        setCurrentUser(data.data)
      }
    })()
  }, [isSuccess])

  if (isPending) return <p>Loading...</p>

  if (isSuccess) return children
}

export default AuthGuard;
