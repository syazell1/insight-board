import { useAxios } from "@/hooks/use-axios"
import { useMutation, useQueryClient } from "@tanstack/react-query"
import type { TApiServiceForm } from "../api-service.types";
import { currentUserStore } from "@/features/auth/stores/currentUserStore";

export const useAddApi = () => {
    const { currentUser } = currentUserStore();
    const api = useAxios();
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: async (data: TApiServiceForm) => {
            await api.post("/api", data);
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ["api", { name: currentUser }] });
        }
    })
}