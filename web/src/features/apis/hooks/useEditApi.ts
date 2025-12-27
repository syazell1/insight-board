import { useAxios } from "@/hooks/use-axios"
import { useMutation, useQueryClient } from "@tanstack/react-query"
import type { TApiServiceForm } from "../api-service.types";
import { currentUserStore } from "@/features/auth/stores/currentUserStore";

export const useEditApi = () => {
  const { currentUser } = currentUserStore();
  const queryClient = useQueryClient();
  const api = useAxios();

  return useMutation({
    mutationFn: async (data: TApiServiceForm) => {
      await api.post("/api", data);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["api", { name: currentUser }] });
    }
  })
}
