import { useAxios } from "@/hooks/use-axios";
import { useQuery } from "@tanstack/react-query"

export interface ICurrentUser {
  id: string,
  username: string
}

export const useGetCurrentUser = () => {
  const client = useAxios();

  return useQuery({
    queryFn: async () => {
      const data = await client.get<ICurrentUser>("/auth/current-user");
      return data;
    },
    queryKey: ["profile"]
  })
}
