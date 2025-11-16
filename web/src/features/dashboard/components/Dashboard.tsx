import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { AlertCircle, CheckCircle2, Cpu, HardDrive, Globe, Server } from "lucide-react";
import { LineChart, Line, AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/AppSideBar";

// Mock data
const cpuData = [
  { time: '00:00', value: 45 },
  { time: '04:00', value: 52 },
  { time: '08:00', value: 68 },
  { time: '12:00', value: 75 },
  { time: '16:00', value: 82 },
  { time: '20:00', value: 58 },
];

const memoryData = [
  { time: '00:00', value: 2.1 },
  { time: '04:00', value: 2.3 },
  { time: '08:00', value: 3.5 },
  { time: '12:00', value: 4.2 },
  { time: '16:00', value: 4.8 },
  { time: '20:00', value: 3.1 },
];

const services = [
  { name: 'API Gateway', status: 'healthy', uptime: '99.98%', responseTime: '42ms' },
  { name: 'Auth Service', status: 'healthy', uptime: '99.95%', responseTime: '38ms' },
  { name: 'Database', status: 'warning', uptime: '99.82%', responseTime: '125ms' },
  { name: 'Cache Layer', status: 'healthy', uptime: '99.99%', responseTime: '12ms' },
];

const logs = [
  { time: '14:32:15', level: 'info', service: 'API', message: 'Request processed successfully' },
  { time: '14:31:52', level: 'warning', service: 'Database', message: 'High query latency detected' },
  { time: '14:30:28', level: 'info', service: 'Auth', message: 'User authentication successful' },
  { time: '14:29:45', level: 'error', service: 'Cache', message: 'Cache miss rate exceeding threshold' },
  { time: '14:28:33', level: 'info', service: 'API', message: 'Health check passed' },
];

const Dashboard = () => {
  return (
    <SidebarProvider>
      <div className="min-h-screen flex w-full bg-background">
        <AppSidebar />
        
        <main className="flex-1">
          {/* Header */}
          <header className="border-b border-border bg-card/50 backdrop-blur-sm sticky top-0 z-20">
            <div className="container mx-auto px-4 py-4 flex items-center gap-2">
              <SidebarTrigger />
              <h1 className="text-2xl font-bold">Dashboard</h1>
            </div>
          </header>

          <div className="container mx-auto px-4 py-8">
        {/* Metrics Overview */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
          <Card className="border-border bg-card">
            <CardHeader className="pb-3">
              <CardTitle className="text-sm font-medium text-muted-foreground flex items-center gap-2">
                <Cpu className="h-4 w-4" />
                CPU Usage
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-3xl font-bold">58%</div>
              <p className="text-xs text-success mt-1">↓ 12% from last hour</p>
            </CardContent>
          </Card>

          <Card className="border-border bg-card">
            <CardHeader className="pb-3">
              <CardTitle className="text-sm font-medium text-muted-foreground flex items-center gap-2">
                <HardDrive className="h-4 w-4" />
                Memory
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-3xl font-bold">3.1 GB</div>
              <p className="text-xs text-warning mt-1">↑ 5% from last hour</p>
            </CardContent>
          </Card>

          <Card className="border-border bg-card">
            <CardHeader className="pb-3">
              <CardTitle className="text-sm font-medium text-muted-foreground flex items-center gap-2">
                <Globe className="h-4 w-4" />
                Requests/min
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-3xl font-bold">1,247</div>
              <p className="text-xs text-success mt-1">↑ 8% from last hour</p>
            </CardContent>
          </Card>

          <Card className="border-border bg-card">
            <CardHeader className="pb-3">
              <CardTitle className="text-sm font-medium text-muted-foreground flex items-center gap-2">
                <Server className="h-4 w-4" />
                Services
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-3xl font-bold">4/4</div>
              <p className="text-xs text-success mt-1">All systems operational</p>
            </CardContent>
          </Card>
        </div>

        {/* Charts */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
          <Card className="border-border bg-card">
            <CardHeader>
              <CardTitle>CPU Usage (24h)</CardTitle>
              <CardDescription>Average CPU utilization across all instances</CardDescription>
            </CardHeader>
            <CardContent>
              <ResponsiveContainer width="100%" height={200}>
                <AreaChart data={cpuData}>
                  <defs>
                    <linearGradient id="colorCpu" x1="0" y1="0" x2="0" y2="1">
                      <stop offset="5%" stopColor="hsl(var(--primary))" stopOpacity={0.3}/>
                      <stop offset="95%" stopColor="hsl(var(--primary))" stopOpacity={0}/>
                    </linearGradient>
                  </defs>
                  <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" />
                  <XAxis dataKey="time" stroke="hsl(var(--muted-foreground))" fontSize={12} />
                  <YAxis stroke="hsl(var(--muted-foreground))" fontSize={12} />
                  <Tooltip 
                    contentStyle={{ 
                      backgroundColor: 'hsl(var(--card))', 
                      border: '1px solid hsl(var(--border))',
                      borderRadius: '6px'
                    }} 
                  />
                  <Area type="monotone" dataKey="value" stroke="hsl(var(--primary))" fill="url(#colorCpu)" strokeWidth={2} />
                </AreaChart>
              </ResponsiveContainer>
            </CardContent>
          </Card>

          <Card className="border-border bg-card">
            <CardHeader>
              <CardTitle>Memory Usage (24h)</CardTitle>
              <CardDescription>Memory consumption in gigabytes</CardDescription>
            </CardHeader>
            <CardContent>
              <ResponsiveContainer width="100%" height={200}>
                <LineChart data={memoryData}>
                  <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" />
                  <XAxis dataKey="time" stroke="hsl(var(--muted-foreground))" fontSize={12} />
                  <YAxis stroke="hsl(var(--muted-foreground))" fontSize={12} />
                  <Tooltip 
                    contentStyle={{ 
                      backgroundColor: 'hsl(var(--card))', 
                      border: '1px solid hsl(var(--border))',
                      borderRadius: '6px'
                    }} 
                  />
                  <Line type="monotone" dataKey="value" stroke="hsl(var(--success))" strokeWidth={2} dot={{ fill: 'hsl(var(--success))' }} />
                </LineChart>
              </ResponsiveContainer>
            </CardContent>
          </Card>
        </div>

        {/* Services Status */}
        <Card className="border-border bg-card mb-8">
          <CardHeader>
            <CardTitle>Service Status</CardTitle>
            <CardDescription>Real-time status of all monitored services</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {services.map((service) => (
                <div key={service.name} className="flex items-center justify-between p-4 rounded-lg border border-border bg-background/50">
                  <div className="flex items-center gap-3">
                    {service.status === 'healthy' ? (
                      <CheckCircle2 className="h-5 w-5 text-success" />
                    ) : (
                      <AlertCircle className="h-5 w-5 text-warning" />
                    )}
                    <div>
                      <div className="font-medium">{service.name}</div>
                      <div className="text-sm text-muted-foreground">Uptime: {service.uptime}</div>
                    </div>
                  </div>
                  <div className="flex items-center gap-4">
                    <div className="text-right">
                      <div className="text-sm text-muted-foreground">Response Time</div>
                      <div className="font-medium">{service.responseTime}</div>
                    </div>
                    <Badge variant={service.status === 'healthy' ? 'default' : 'secondary'} className={service.status === 'healthy' ? 'bg-success' : 'bg-warning'}>
                      {service.status}
                    </Badge>
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>

        {/* Recent Logs */}
        <Card className="border-border bg-card">
          <CardHeader>
            <CardTitle>Recent Logs</CardTitle>
            <CardDescription>Latest system events and messages</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-2 font-mono text-sm">
              {logs.map((log, i) => (
                <div key={i} className="flex items-start gap-3 p-3 rounded border border-border bg-background/50 hover:bg-background/70 transition-colors">
                  <span className="text-muted-foreground shrink-0">{log.time}</span>
                  <Badge 
                    variant="outline" 
                    className={`shrink-0 ${
                      log.level === 'error' ? 'border-destructive text-destructive' :
                      log.level === 'warning' ? 'border-warning text-warning' :
                      'border-info text-info'
                    }`}
                  >
                    {log.level}
                  </Badge>
                  <span className="text-primary shrink-0">[{log.service}]</span>
                  <span className="text-foreground">{log.message}</span>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
          </div>
        </main>
      </div>
    </SidebarProvider>
  );
};

export default Dashboard;
