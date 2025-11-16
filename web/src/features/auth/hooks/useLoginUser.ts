import { useMutation } from "@tanstack/react-query"
import type { IAuthResponse, TLoginUser } from "../auth.types"
import { useAuth } from "@/context/AuthContext";
import { api } from "@/lib/api";
import { useNavigate } from "@tanstack/react-router";



export const useLoginUser = () => {
  const { setAccessToken } = useAuth();
  const navigate = useNavigate();

  return useMutation({
    mutationFn: async (data: TLoginUser) => {
      const res = await api.post<IAuthResponse>("/auth/login", data);

      return res.data;
    },
    onSuccess: (data) => {
      setAccessToken(data.access_token);

      navigate({ to: "/dashboard", replace: true });
    }
  })

}
