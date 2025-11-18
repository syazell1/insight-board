import Services from '@/features/apis/components/ApiList'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/(protected)/services')({
  component: RouteComponent,
})

function RouteComponent() {
  return <Services />
}
