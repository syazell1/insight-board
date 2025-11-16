import AuthForm from '@/features/auth/components/AuthForm'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/(auth)/auth')({
  component: RouteComponent,
})

function RouteComponent() {
  return <AuthForm />
}
