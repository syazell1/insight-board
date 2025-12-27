## InsightBoard — API & Server Monitoring Dashboard

**InsightBoard** is a **DevOps-style monitoring dashboard** that collects and visualizes system metrics, logs, and uptime for APIs, microservices, or apps.


It helps developers track:
- Server uptime
- API latency
- Error rates
- Requests per minute

### 1. 🌐 **API Monitoring**
Each user can register their APIs to monitor.  
InsightBoard periodically:
- Sends HTTP requests (ping checks)    
- Measures latency, status code, and uptime %
- Stores the results in a metrics DB    

You can display:
- Average latency (ms)
- Success rate (%)
- Downtime logs

---

### 2. ⚙️ **System Metrics **
- Requests per second
- Error count
- CPU, RAM, Disk usage
---

### 3. 📊 **Dashboard Visualization**

A frontend dashboard showing:
- Line charts (latency over time)
- Bar charts (requests per hour/day)
- Pie charts (error types or API distribution)
    
---

### 4. 👤 **User Accounts**
Basic multi-user support:
- Each user adds their own APIs to monitor.
- Each has a dashboard view. 


---

### 🧰 **Tech Stack**
- Backend - Rust
- Database - PostgreSQL
- Frontend - React/Tanstack Router