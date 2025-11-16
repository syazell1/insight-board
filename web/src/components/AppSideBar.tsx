import { Activity, LayoutDashboard, Server, FileText, Settings, LogOut } from "lucide-react";
// import { NavLink } from "@/components/NavLink";
import {
    Sidebar,
    SidebarContent,
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarFooter,
    SidebarHeader,
    useSidebar,
} from "@/components/ui/sidebar";
import { Link } from "@tanstack/react-router";

const navigationItems = [
    { title: "Dashboard", url: "/dashboard", icon: LayoutDashboard },
    { title: "Services", url: "/services", icon: Server },
    { title: "Logs", url: "/logs", icon: FileText },
    { title: "Settings", url: "/settings", icon: Settings },
];

export function AppSidebar() {
    const { state } = useSidebar();
    //   const location = useLocation();
    //   const currentPath = location.pathname;
    const collapsed = state === "collapsed";

    return (
        <Sidebar collapsible="icon" className="border-r border-border z-100">
            <SidebarHeader className="border-b border-border p-4">
                <div className="flex items-center gap-2">
                    <Activity className="h-6 w-6 text-primary" />
                    {!collapsed && <span className="font-bold text-lg">InsightBoard</span>}
                </div>
            </SidebarHeader>

            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupLabel>Navigation</SidebarGroupLabel>
                    <SidebarGroupContent>
                        <SidebarMenu>
                            {navigationItems.map((item) => (
                                <SidebarMenuItem key={item.title}>
                                    <SidebarMenuButton asChild>
                                        <Link
                                            to={item.url}
                                            className="hover:bg-muted/50"
                                        //   activeClassName="bg-muted text-primary font-medium"
                                        >
                                            <item.icon className="h-4 w-4" />
                                            {!collapsed && <span>{item.title}</span>}
                                        </Link>
                                    </SidebarMenuButton>
                                </SidebarMenuItem>
                            ))}
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>

            <SidebarFooter className="border-t border-border p-2">
                <SidebarMenu>
                    <SidebarMenuItem>
                        <SidebarMenuButton asChild>
                            <Link to="/" className="hover:bg-muted/50 text-destructive">
                                <LogOut className="h-4 w-4" />
                                {!collapsed && <span>Logout</span>}
                            </Link>
                        </SidebarMenuButton>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarFooter>
        </Sidebar>
    );
}