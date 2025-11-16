import { create } from 'zustand'
import { type ICurrentUser } from '../hooks/useGetCurrentUser'

interface ICurrentUserStore {
  currentUser: ICurrentUser,
  setCurrentUser: (data: ICurrentUser) => void
}

export const currentUserStore = create<ICurrentUserStore>(set => ({
  currentUser: { id: "", username: "" },
  setCurrentUser: (data) => set(() => ({ currentUser: data }))
}))


