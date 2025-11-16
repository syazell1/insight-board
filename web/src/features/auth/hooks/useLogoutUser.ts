import { useAuth } from "@/context/AuthContext";
import { api } from "@/lib/api";
import { useMutation } from "@tanstack/react-query"
import { currentUserStore } from "../stores/currentUserStore";
import { useNavigate } from "@tanstack/react-router";

export const useLogoutUser = () => {
  const { setAccessToken } = useAuth();
  const { setCurrentUser } = currentUserStore()
  const navigate = useNavigate();

  return useMutation({
    mutationFn: async () => {
      await api.post("/auth/logout");
    },
    onSuccess: () => {
      setAccessToken(""); // store in memory
      setCurrentUser({
        id: "",
        username: ""
      })
      navigate({ to: "/login", replace: true });
    }
  })
}
