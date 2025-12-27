import { currentUserStore } from "@/features/auth/stores/currentUserStore";
import { useAxios } from "@/hooks/use-axios";
import { useMutation, useQueryClient } from "@tanstack/react-query"
import { toast } from "sonner";

export const useDeleteApi = () => {
  const { currentUser } = currentUserStore();
  const queryClient = useQueryClient();
  const api = useAxios();

  return useMutation({
    mutationFn: async (id: string) => {
      return await api.delete(`/api/${id}`);
    },
    onSuccess: () => {
      toast.success("API Service Successfully Deleted.");

      queryClient.invalidateQueries({ queryKey: ["api", { name: currentUser }] });
    }
  })
}
