import { createFileRoute, Link } from '@tanstack/react-router'


import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Activity, BarChart3, Bell, Database, Eye, Zap } from "lucide-react";

export const Route = createFileRoute('/')({
  component: Index,
})
function Index() {
  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <header className="border-b border-border bg-card/50 backdrop-blur-sm sticky top-0 z-50">
        <div className="container mx-auto px-4 py-4 flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Activity className="h-8 w-8 text-primary" />
            <h1 className="text-2xl font-bold">InsightBoard</h1>
          </div>
          <Link to="/auth">
            <Button variant="outline">Sign In</Button>
          </Link>
        </div>
      </header>

      {/* Hero Section */}
      <section className="container mx-auto px-4 py-20 text-center">
        <div className="max-w-4xl mx-auto space-y-8">
          <div className="inline-block">
            <div className="bg-gradient-hero bg-clip-text text-transparent">
              <h2 className="text-5xl md:text-6xl font-bold mb-4">
                Monitor Everything.<br />Optimize Anywhere.
              </h2>
            </div>
          </div>
          <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
            DevOps-style monitoring dashboard that collects and visualizes system metrics,
            logs, and uptime for your APIs, microservices, and applications in real-time.
          </p>
          <div className="flex gap-4 justify-center">
            <Link to="/auth">
              <Button size="lg" className="bg-primary hover:bg-primary/90">
                Get Started Free
              </Button>
            </Link>
            <Link to="/dashboard">
              <Button size="lg" variant="outline">
                View Demo
              </Button>
            </Link>
          </div>
        </div>
      </section>

      {/* Features Grid */}
      <section className="container mx-auto px-4 py-20">
        <div className="text-center mb-12">
          <h3 className="text-3xl font-bold mb-4">Powerful Monitoring Features</h3>
          <p className="text-muted-foreground">Everything you need to keep your systems running smoothly</p>
        </div>
        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
          <Card className="border-border bg-card hover:border-primary/50 transition-colors">
            <CardHeader>
              <Activity className="h-10 w-10 text-primary mb-2" />
              <CardTitle>Real-Time Metrics</CardTitle>
              <CardDescription>
                Monitor CPU, memory, and network usage across all your services in real-time
              </CardDescription>
            </CardHeader>
          </Card>

          <Card className="border-border bg-card hover:border-primary/50 transition-colors">
            <CardHeader>
              <BarChart3 className="h-10 w-10 text-success mb-2" />
              <CardTitle>Advanced Analytics</CardTitle>
              <CardDescription>
                Visualize trends with beautiful charts and identify performance bottlenecks
              </CardDescription>
            </CardHeader>
          </Card>

          <Card className="border-border bg-card hover:border-primary/50 transition-colors">
            <CardHeader>
              <Bell className="h-10 w-10 text-warning mb-2" />
              <CardTitle>Smart Alerts</CardTitle>
              <CardDescription>
                Get notified instantly when metrics exceed thresholds or services go down
              </CardDescription>
            </CardHeader>
          </Card>

          <Card className="border-border bg-card hover:border-primary/50 transition-colors">
            <CardHeader>
              <Database className="h-10 w-10 text-info mb-2" />
              <CardTitle>Log Aggregation</CardTitle>
              <CardDescription>
                Collect and search logs from all your services in one centralized location
              </CardDescription>
            </CardHeader>
          </Card>

          <Card className="border-border bg-card hover:border-primary/50 transition-colors">
            <CardHeader>
              <Eye className="h-10 w-10 text-primary mb-2" />
              <CardTitle>Uptime Monitoring</CardTitle>
              <CardDescription>
                Track availability and response times for APIs and web services 24/7
              </CardDescription>
            </CardHeader>
          </Card>

          <Card className="border-border bg-card hover:border-primary/50 transition-colors">
            <CardHeader>
              <Zap className="h-10 w-10 text-success mb-2" />
              <CardTitle>Quick Setup</CardTitle>
              <CardDescription>
                Get started in minutes with our simple integration and beautiful dashboards
              </CardDescription>
            </CardHeader>
          </Card>
        </div>
      </section>

      {/* CTA Section */}
      <section className="container mx-auto px-4 py-20">
        <Card className="bg-gradient-hero border-0 text-primary-foreground">
          <CardContent className="p-12 text-center">
            <h3 className="text-3xl font-bold mb-4">Ready to Get Started?</h3>
            <p className="text-lg mb-8 opacity-90">
              Join thousands of teams monitoring their infrastructure with InsightBoard
            </p>
            <Link to="/auth">
              <Button size="lg" variant="secondary" className="bg-background text-foreground hover:bg-background/90">
                Start Monitoring Now
              </Button>
            </Link>
          </CardContent>
        </Card>
      </section>

      {/* Footer */}
      <footer className="border-t border-border bg-card/50">
        <div className="container mx-auto px-4 py-8 text-center text-muted-foreground">
          <p>&copy; 2025 InsightBoard. All rights reserved.</p>
        </div>
      </footer>
    </div>
  );
}
