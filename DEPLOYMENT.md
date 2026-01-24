# MicroRoulette Deployment Guide

This guide explains how to deploy MicroRoulette for hackathon judging. Two deployment options are available:

## Option 1: Docker (Recommended - Real Blockchain)

**Use this for judges to verify real blockchain integration.**

### Prerequisites
- Docker and Docker Compose installed
- 2GB free RAM
- Port 8080 and 8082 available

### Steps

```bash
# Clone repository
git clone [your-repo-url]
cd micro-roulette

# Start with Docker
docker compose up --build
```

### What Happens
1. Rust container builds Linera CLI v0.15.8
2. Wallet initialized with Conway testnet faucet
3. Smart contracts loaded (pre-compiled WASM)
4. GraphQL service starts on port 8082
5. Frontend served on port 8080

### Verification
Open http://localhost:8080

You should see:
- **Chain ID**: `781078b5a05e20fb1cd13c06622ccc91f813d112f020816e799a9ec1ba4298dc`
- **App ID**: `9b16ccbe34c686f959ad6d3ebe9dde35a1ab7cf73a99a470feae0b082be59059`
- **Network**: Linera Conway Testnet

These values prove real blockchain connection, not demo mode.

### Direct GraphQL Test
```bash
curl -X POST http://localhost:8082/chains/781078b5a05e20fb1cd13c06622ccc91f813d112f020816e799a9ec1ba4298dc/applications/9b16ccbe34c686f959ad6d3ebe9dde35a1ab7cf73a99a470feae0b082be59059 \
  -H "Content-Type: application/json" \
  -d '{"query": "{ tableStatus spinNumber }"}'
```

Expected response:
```json
{
  "data": {
    "tableStatus": "Open",
    "spinNumber": 0
  }
}
```

---

## Option 2: Vercel (Demo Mode - Quick Preview)

**Use this for quick UI/UX preview without blockchain setup.**

### Deploy to Vercel

[![Deploy with Vercel](https://vercel.com/button)](https://vercel.com/new/clone?repository-url=https://github.com/[your-repo])

Or manually:

```bash
# Install Vercel CLI
npm i -g vercel

# Deploy
cd micro-roulette
vercel --prod
```

### Environment Variables (Vercel Dashboard)

```
VITE_LINERA_FAUCET_URL=https://faucet.testnet-conway.linera.net
VITE_APP_ID=9b16ccbe34c686f959ad6d3ebe9dde35a1ab7cf73a99a470feae0b082be59059
VITE_CHAIN_ID=781078b5a05e20fb1cd13c06622ccc91f813d112f020816e799a9ec1ba4298dc
VITE_LINERA_SERVICE_PORT=8082
VITE_DEMO_MODE=true
```

**Note**: `VITE_DEMO_MODE=true` enables client-side simulation. No real blockchain connection. For blockchain verification, use Docker.

---

## Option 3: Render.com (Free Tier - Static Site)

### Deploy Frontend to Render

1. Go to [render.com](https://render.com)
2. New > Static Site
3. Connect your GitHub repo
4. Build Command: `cd frontend && npm install && npm run build`
5. Publish Directory: `frontend/dist`

### Environment Variables

Add in Render dashboard:
```
VITE_LINERA_FAUCET_URL=https://faucet.testnet-conway.linera.net
VITE_APP_ID=9b16ccbe34c686f959ad6d3ebe9dde35a1ab7cf73a99a470feae0b082be59059
VITE_CHAIN_ID=781078b5a05e20fb1cd13c06622ccc91f813d112f020816e799a9ec1ba4298dc
VITE_DEMO_MODE=true
```

**Note**: Like Vercel, this runs in demo mode. Use Docker for real blockchain.

---

## Comparison

| Method | Blockchain | Setup Time | Free Tier | Best For |
|--------|-----------|------------|-----------|----------|
| **Docker** | ✅ Real (Conway Testnet) | 5 min | ✅ Yes | Judges verifying blockchain integration |
| **Vercel** | ❌ Demo Mode | 2 min | ✅ Yes | Quick UI/UX preview |
| **Render** | ❌ Demo Mode | 3 min | ✅ Yes | Alternative to Vercel |

---

## For Hackathon Judges

**To verify this is a real blockchain application:**

1. Use the Docker deployment option
2. Check the UI header shows Chain ID and App ID
3. Run the GraphQL curl command to query the blockchain directly
4. Place a bet and spin - observe the blockchain transaction
5. Check browser DevTools > Network tab for GraphQL requests to port 8082

**Red flags for demo mode:**
- No Chain ID displayed
- No GraphQL requests in Network tab
- Console shows "DEMO_MODE enabled"

**Green flags for real blockchain:**
- Chain ID: `781078b5a05e20fb1cd13c06622ccc91f813d112f020816e799a9ec1ba4298dc`
- GraphQL endpoint: `localhost:8082/chains/.../applications/...`
- Console shows "Linera service connected"

---

## Troubleshooting

### Docker build fails
```bash
# Clean and rebuild
docker compose down -v
docker compose up --build --force-recreate
```

### Port conflicts
```bash
# Check what's using ports 8080/8082
lsof -i :8080
lsof -i :8082

# Kill the processes
kill -9 [PID]
```

### Wallet initialization fails
- Check internet connection to https://faucet.testnet-conway.linera.net
- Conway testnet may be down - check Linera Discord for updates

### Frontend shows "Connecting..." forever
- Ensure Linera service is running on port 8082
- Check Docker logs: `docker compose logs roulette`
- Verify GraphQL endpoint is accessible

---

## Production URLs (After Deployment)

- **Live Demo (Vercel)**: [URL will be here after deployment]
- **GitHub Repo**: [your-repo-url]
- **Documentation**: [README.md](./README.md)
- **Test Artifacts**: [test-artifacts/](./test-artifacts/)

---

## Next Steps After Deployment

1. Update README.md with live URLs
2. Test both Docker and Vercel deployments
3. Take screenshots for hackathon submission
4. Prepare demo video showing blockchain verification
5. Document any deployment issues for judges

---

Built for the Linera Hackathon 🏆
