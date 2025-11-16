import AuthGuard from '@/features/auth/components/AuthGuard'
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/(protected)')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <AuthGuard>
      <Outlet />
    </AuthGuard>
  ) 
}
