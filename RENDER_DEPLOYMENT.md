# Deploying BDLD API on Render.com

This guide walks you through deploying the Bitcoin Digital Labor Derivative REST API on Render.com's free tier.

## Prerequisites

- GitHub account with this repository
- Render.com account (free tier available)
- Git configured locally (for testing)

## Quick Start (1-Click Deploy)

### Option 1: Deploy via Render Dashboard

1. **Create a Render Account**
   - Visit [render.com](https://render.com)
   - Sign up with GitHub (recommended for easy integration)

2. **Create a New Web Service**
   - Click "New +" → "Web Service"
   - Connect your GitHub repository: `dfeen87/Bitcoin-Digital-Labor-Derivative`
   - Authorize Render to access the repository

3. **Configure the Service**
   
   **Basic Settings:**
   - **Name:** `bdld-api` (or your preferred name)
   - **Region:** Select closest to your users
   - **Branch:** `main` (or your deployment branch)
   - **Root Directory:** Leave empty
   - **Runtime:** `Docker`
   
   **Build Settings:**
   - **Build Command:** (leave empty - Docker handles this)
   - **Docker Command:** (leave empty - uses Dockerfile CMD)
   
   **Plan:**
   - Select **Free** tier
   
4. **Set Environment Variables** (Optional)
   
   Click "Advanced" → "Add Environment Variable"
   
   Recommended variables:
   ```
   BDLD_ENV=production
   BDLD_NODE_ID=render-node-01
   BDLD_LOG_LEVEL=info
   BDLD_PORT=8080
   ```
   
   See `.env.example` for all available options.

5. **Deploy**
   - Click "Create Web Service"
   - Render will automatically:
     - Clone your repository
     - Build the Docker image
     - Deploy to a public URL
     - Set up HTTPS automatically
   
6. **Access Your API**
   - Your API will be available at: `https://bdld-api.onrender.com`
   - Health check: `https://bdld-api.onrender.com/health`
   - Status: `https://bdld-api.onrender.com/status`

## Configuration Details

### Build Configuration

Render automatically uses the `Dockerfile` in the repository root:

- **Build Command:** Docker build (automatic)
- **Start Command:** `/app/api-server` (from Dockerfile)
- **Port:** 8080 (Render standard)
- **Health Check Path:** `/health`

### Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `BDLD_ENV` | No | `production` | Environment mode |
| `BDLD_PORT` | No | `8080` | API server port |
| `BDLD_NODE_ID` | No | Auto-generated | Unique node identifier |
| `BDLD_LOG_LEVEL` | No | `info` | Logging verbosity |
| `BDLD_JWT_SECRET` | No | _(empty)_ | JWT secret (auth disabled if empty) |
| `RUST_LOG` | No | `info` | Rust logging filter |

**Note:** Render automatically sets `PORT` environment variable. The application uses `BDLD_PORT` but falls back to Render's `PORT` if needed.

### Health Checks

Render automatically monitors your service using the health check endpoint:

- **Path:** `/health`
- **Expected Response:** `200 OK` with `{"status":"healthy","version":"..."}`
- **Interval:** Every 30 seconds
- **Timeout:** 3 seconds
- **Retries:** 3 before marking unhealthy

## Deployment from Git Push

After initial setup, Render automatically redeploys when you push to the configured branch:

```bash
# Make changes to your code
git add .
git commit -m "Update API endpoints"
git push origin main

# Render automatically:
# 1. Detects the push
# 2. Rebuilds the Docker image
# 3. Deploys the new version
# 4. Performs health checks
# 5. Routes traffic to new version
```

## Viewing Logs

### Real-time Logs

1. Go to your Render dashboard
2. Select your `bdld-api` service
3. Click "Logs" tab
4. View real-time application logs

### Log Levels

Control log verbosity with `BDLD_LOG_LEVEL`:

- `error`: Only errors
- `warn`: Warnings and errors
- `info`: General information (recommended for production)
- `debug`: Detailed debugging information
- `trace`: Very verbose (not recommended for production)

## API Endpoints

Once deployed, your API exposes the following endpoints:

### Core Endpoints
- `GET /health` - Service health check
- `GET /status` - Version, uptime, configuration

### Labor/Derivative Endpoints
- `GET /labor/state` - Full derivative state snapshot
- `GET /labor/history?page=0&page_size=20` - Paginated labor-value history
- `POST /labor/apply` - Apply new labor input (bounded)
- `GET /labor/value` - Current BDLD labor value
- `GET /labor/volatility` - Volatility model

### Bitcoin Peg
- `GET /btc/peg` - BTC peg ratio and stability model

### Legacy API (v1)
- `GET /api/v1/rbi` - Recession Bypass Index
- `GET /api/v1/pool/balance` - Pool balance
- `GET /api/v1/participants/:id/dividend` - Calculate dividend
- `GET /api/v1/participants/:id/velocity` - Velocity data

## Testing Your Deployment

### Using curl

```bash
# Health check
curl https://bdld-api.onrender.com/health

# Get status
curl https://bdld-api.onrender.com/status

# Get labor state
curl https://bdld-api.onrender.com/labor/state

# Apply labor input
curl -X POST https://bdld-api.onrender.com/labor/apply \
  -H "Content-Type: application/json" \
  -d '{
    "participant_id": "alice",
    "labor_value_sats": 50000000,
    "duration_days": 90
  }'

# Get BTC peg ratio
curl https://bdld-api.onrender.com/btc/peg
```

### Using JavaScript/Fetch

```javascript
// Get status
fetch('https://bdld-api.onrender.com/status')
  .then(res => res.json())
  .then(data => console.log(data));

// Apply labor input
fetch('https://bdld-api.onrender.com/labor/apply', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    participant_id: 'bob',
    labor_value_sats: 100000000,
    duration_days: 365
  })
})
  .then(res => res.json())
  .then(data => console.log(data));
```

## Scaling (Free Tier Limitations)

### Free Tier Specifications
- **RAM:** 512 MB
- **CPU:** Shared
- **Bandwidth:** Limited
- **Sleep:** Services sleep after 15 minutes of inactivity
- **Cold Start:** 30-60 seconds when service wakes

### Performance Tips

1. **Keep Service Warm**
   - Use a service like UptimeRobot to ping `/health` every 14 minutes
   - Prevents service from sleeping

2. **Optimize for Cold Starts**
   - First request after sleep may take 30-60 seconds
   - Subsequent requests are fast

3. **Rate Limiting**
   - API is configured for 100 requests/minute
   - Adjust via `BDLD_RATE_LIMIT_PER_MINUTE` if needed

### Upgrading to Paid Tier

For production workloads, consider upgrading:
- **Starter ($7/month):** Always-on, no cold starts
- **Standard ($25/month):** More resources, better performance

## Monitoring and Maintenance

### Service Monitoring

1. **Render Dashboard**
   - View service status
   - Check deployment history
   - Monitor resource usage

2. **Health Endpoint**
   - Monitor `/health` endpoint
   - Set up external monitoring (UptimeRobot, Pingdom, etc.)

3. **Logs**
   - Check logs regularly for errors
   - Use log filtering in Render dashboard

### Common Issues

**Service Won't Start**
- Check build logs for compilation errors
- Verify Dockerfile is correct
- Ensure port 8080 is exposed

**Health Check Failing**
- Verify `/health` endpoint returns 200 OK
- Check application logs for startup errors
- Ensure service binds to `0.0.0.0:8080`

**Slow Response Times**
- Service may be sleeping (free tier)
- Consider upgrading to always-on plan
- Use UptimeRobot to keep service warm

## Security Considerations

### Default Security Features

✅ **Enabled by Default:**
- HTTPS (automatic via Render)
- CORS (permissive by default)
- Rate limiting (100 req/min)
- Non-root Docker user
- No external Bitcoin node required

❌ **Disabled by Default:**
- JWT authentication (optional)

### Enabling JWT Authentication

If you need authentication:

1. Generate a secure secret:
   ```bash
   openssl rand -hex 32
   ```

2. Add to Render environment variables:
   ```
   BDLD_JWT_SECRET=your_generated_secret_here
   BDLD_JWT_ENABLED=true
   ```

3. Restart your service

**Note:** JWT authentication is NOT implemented in the current version. This is a placeholder for future enhancement.

### Production Security Checklist

- [ ] Use HTTPS (automatic on Render)
- [ ] Set unique `BDLD_NODE_ID`
- [ ] Configure appropriate rate limits
- [ ] Monitor logs for suspicious activity
- [ ] Keep dependencies updated
- [ ] Don't commit secrets to Git

## Costs

### Free Tier
- **Price:** $0/month
- **Limitations:** 
  - Service sleeps after 15 min inactivity
  - 512 MB RAM
  - Shared CPU
  - 750 hours/month (enough for 1 service)

### Paid Tiers
- **Starter:** $7/month - Always on, no sleep
- **Standard:** $25/month - More resources
- **Pro:** $85/month - Dedicated resources

**Recommendation:** Start with free tier, upgrade if needed.

## Troubleshooting

### Build Failures

**Problem:** Docker build fails

**Solutions:**
1. Check Dockerfile syntax
2. Verify Rust version compatibility
3. Check cargo build logs
4. Ensure all dependencies are in Cargo.toml

### Runtime Errors

**Problem:** Service crashes after deployment

**Solutions:**
1. Check application logs in Render dashboard
2. Verify environment variables are set correctly
3. Test Docker image locally:
   ```bash
   docker build -t bdld-api .
   docker run -p 8080:8080 bdld-api
   ```

### Connection Issues

**Problem:** Cannot connect to API

**Solutions:**
1. Verify service is running in Render dashboard
2. Check health check status
3. Ensure you're using HTTPS (not HTTP)
4. Check if service is sleeping (free tier)

## Support

- **Documentation:** See `README.md` in repository
- **Issues:** Open an issue on GitHub
- **Render Support:** [render.com/docs](https://render.com/docs)

## Next Steps

After deployment:

1. ✅ Test all endpoints
2. ✅ Set up monitoring (UptimeRobot, etc.)
3. ✅ Configure custom domain (optional)
4. ✅ Integrate with your application
5. ✅ Monitor logs and performance
6. ✅ Plan for scaling if needed

---

**Congratulations!** Your BDLD API is now live and ready to serve Bitcoin-denominated digital labor derivative calculations to the world. 🚀
