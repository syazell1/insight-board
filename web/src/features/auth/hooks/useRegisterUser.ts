import { useAuth } from "@/context/AuthContext";
import { api } from "@/lib/api";
import { useMutation } from "@tanstack/react-query"
import type { IAuthResponse, TRegisterUser } from "../auth.types";
import { currentUserStore } from "../stores/currentUserStore";
import { useNavigate } from "@tanstack/react-router";

export const useRegisterUser = () => {
  const { setAccessToken } = useAuth();
  const { setCurrentUser } = currentUserStore()
  const navigate = useNavigate();

  return useMutation({
    mutationFn: async (data: TRegisterUser) => {
      const res = await api.post<IAuthResponse>("/auth/register", data);

      return { auth: res.data, username: data.username };
    },
    onSuccess: ({ auth, username }) => {
      setAccessToken(auth.access_token);
      setCurrentUser({
        id: auth.id,
        username
      })

      navigate({ to: "/dashboard", replace: true });
    }
  })
}
