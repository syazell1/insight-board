import Dashboard from '@/features/dashboard/components/Dashboard'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/(protected)/dashboard')({
  component: RouteComponent,
})

function RouteComponent() {
  return <Dashboard /> 
}
