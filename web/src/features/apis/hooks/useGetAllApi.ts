import { currentUserStore } from "@/features/auth/stores/currentUserStore";
import { useAxios } from "@/hooks/use-axios"
import { useQuery } from "@tanstack/react-query"

interface IApiData {
  id: string,
  name: string,
  url: string,
  interval_seconds: number,
  is_active: boolean,
  created_at: string
}

export const useGetAllApi = () => {
  const api = useAxios();
  const { currentUser } = currentUserStore();


  return useQuery({
    queryFn: async () => {
      const res = await api.get<IApiData[]>("/api");

      return res.data
    },
    queryKey: ["api", { name: currentUser }]
  })
}
