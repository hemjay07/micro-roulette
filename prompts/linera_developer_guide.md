# Linera Developer Guide - Extracted from Discord

*Compiled from Linera Discord #dev-chat discussions (Jan 2026)*

---

## Table of Contents
1. [Buildathon Overview](#buildathon-overview)
2. [Judging Criteria & Integration Scoring](#judging-criteria--integration-scoring)
3. [Technical Best Practices](#technical-best-practices)
4. [Common Pitfalls to Avoid](#common-pitfalls-to-avoid)
5. [Architecture Patterns](#architecture-patterns)
6. [Resources & Links](#resources--links)

---

## Buildathon Overview

### Wave 6 (Final Wave)
- **Deadline:** February 2, 2026
- **Prize Pool:** $13,000 USDC
- **Total Buildathon Prize Pool:** $50,000 USDC across 6 waves

### Minimum Requirements
1. Working Linera contract written in **Rust**
2. Application must **compile and run**
3. Contract must be **deployed and functional** (not just present in code)

### Key Quote from James Kay (Judge):
> "In the final Wave 6 judges will place more emphasis on implementation."

---

## Judging Criteria & Integration Scoring

### The 5 Judging Dimensions
1. **Working Demo & Functionality**
2. **Linera Tech Stack Integration** (most important)
3. **Creativity & User Experience**
4. **Real Use Case & Scalability**
5. **Vision & Roadmap**

### Integration Score (10-point scale)

The judges use a **spectrum/continuum** for integration scoring:

| Score | Integration Level | Description |
|-------|------------------|-------------|
| 1 | Minimal | Just keeping a score on-chain, using chain as verifiable database |
| 5-6 | Moderate | Multiple Linera features used, backend exists but chain is source of truth |
| 9-10 | Deep | Novel features only possible with Linera, protocol-level innovation |

### Key Insight from James Kay:
> "Essentially I think the harder it would be to replace Linera in a project the higher one can expect the integration score to be."

### What Judges Look For:
- **How many user-exposed Linera features** the project uses
- **How important those features are** to the project's core functionality
- Whether Linera could be **removed without fundamentally breaking** the product
- **Real integration in the code**, not just what appears to be integrated

### Red Flags Judges Watch For:
- Misleading documentation
- Code obfuscation to fake integration
- Projects where core logic is server-driven but claims on-chain integration
- "Potemkin Village" frontends (looks integrated but backend is disconnected)

---

## Technical Best Practices

### Contract Architecture

**Single Application, Multiple Domains** (Valid Pattern):
```
contracts/your_app/
  └── Contains one application that encapsulates:
      ├── User registry
      ├── Token/coins economy
      ├── Core game/app logic
      ├── State management
      └── Rewards distribution
```

This is a **deliberate architectural choice** in Linera, not a limitation. One application can handle multiple on-chain domains.

### On-Chain vs Off-Chain Split

**Good Pattern - Chain as Source of Truth:**
```
ON-CHAIN (Linera Contract):
├── User registration
├── XP, coins, levels, game stats
├── Score submissions
├── Critical state changes
├── Rewards distribution
└── All data that needs verification

OFF-CHAIN (Backend Server):
├── Activity feeds (caching)
├── UI performance queries
├── Indexing for fast reads
└── Automation triggers

RULE: Backend cannot mint/modify state on its own.
      All updates require successful on-chain transaction first.
```

### Reading Cross-Chain Data

**Question from developer:**
> "How can I read data from an application chain to display it in the frontend when another user is interacting with it?"

**Solutions:**

1. **Linera Client 0.15.8+** - New connection method allows querying data from different chains

2. **Container + GraphQL** (from @Kvoz):
   > "What I do is run the service in a container and use the full URL to execute the GraphQL queries."

### HTTP API Limitations

From James Kay:
> "The Linera HTTP support is really intended for **read requests** to get data into the system."

> "The real problem you'd have with doing this in the contract is not the list of allowed domains but the fact that **your contract doesn't have access to any secrets** — all the data passed to the contract is public."

**Implication:** For external actions (e.g., buying DNS names, API calls), you need a **client that subscribes to the chain** and has the requisite secrets, acting as a bridge to the outside world.

---

## Common Pitfalls to Avoid

### 1. Shallow Integration
**Problem:** Using Linera just as a "verifiable database" for scores/results
**Solution:** Move core game/app logic on-chain, not just final results

### 2. Server-Driven Logic
**Problem:** All multiplayer/game logic handled by backend server
**Solution:** Create temporary chains for each game session, move logic on-chain

### 3. Missing Testnet Deployment
**Problem:** Project works locally but no `conway_deploy.sh` for testnet
**Solution:** Always include deployment scripts for Testnet Conway

### 4. No Demo Video
**Problem:** Judges can't verify functionality
**Solution:** Include clear demo video in submission (Akindo)

### 5. Hidden/Private Repos During Evaluation
**Problem:** Makes cross-verification impossible
**Solution:** Keep repo public during evaluation period

### 6. Claiming Integration Without Implementation
**Problem:** Documentation says "deep integration" but code shows minimal usage
**Solution:** Be honest about current state; judges read the code

---

## Architecture Patterns

### High-Scoring Project Examples

**1. Multi-Chain Game System:**
```
User Chain ─────┐
                │
Game Chain ─────┼──► Master Chain
                │
Lobby Chain ────┘

Features:
- 4-chain architecture
- Commit-Reveal for provably fair gameplay
- Docker script for instant deployment
```

**2. Proof-of-Work Meme Coin:**
- Uses cutting-edge protocol features
- Customizes underlying consensus mechanism
- Novel use case only possible with Linera

**3. Arcade Hub Pattern (Multi-Module Platform):**
```
Core Contract handles:
├── User registration
├── XP/coins economy
├── Score submissions
├── Prediction markets
├── Rewards distribution
├── Leaderboard state
└── Multiplayer results

Backend (supporting only):
├── Activity feeds
├── Leaderboard caching
└── UI performance queries
```

### Microchain Patterns

**For Games:**
- Create temporary chain for each game session
- Move game logic on-chain (not just results)
- Use cross-chain messaging for multiplayer

**For Prediction Markets:**
- All critical actions (placing bets, resolving, distributing) on-chain
- Backend only for indexing and UI queries
- Chain is single source of truth

---

## Resources & Links

### Official Resources
- **YouTube Channel:** https://www.youtube.com/@linera_io
- **Twitter/X:** https://twitter.com/linera_io
- **Buildathon Info:** https://t.me/linera_official/525409

### Workshop Recordings
| Date | Topic | Link |
|------|-------|------|
| Jan 20, 2026 | Wave 5 Recap | https://www.youtube.com/watch?v=D45Mpw-Kn_M |
| Nov 18, 2025 | Linera x Staex | https://www.youtube.com/watch?v=UAgA0SMDM6Y |
| Jan 6, 2026 | Prediction Markets | [Notion Doc](https://cloudy-raft-0ae.notion.site/Developer-Workshop-Jan-6-2e0bf4cdba9c8085ab30f87b43ed3516) |

### Discord Channels
- **Developer Discussion:** For technical questions between workshops
- **Community Hangout:** Voice channel for live AMAs
- **dev-chat:** Forum for code sharing and discussions

### Key People
- **James Kay** - Technical lead, judge (handles integration scoring)
- **Danny Greene** - Workshop host, community lead
- **GordonG** - Community manager
- **ma2bd (Mathieu)** - Judge, evaluator

---

## Submission Checklist for Wave 6

- [ ] Working Linera contract in Rust
- [ ] Application compiles and runs
- [ ] Contract deployed to Testnet Conway
- [ ] `conway_deploy.sh` script included
- [ ] Demo video showing functionality
- [ ] GitHub repo public during evaluation
- [ ] README with bullet points showing Linera features used
- [ ] Clear documentation of on-chain vs off-chain split

### Pro Tips from the Community

1. **Supply bullet points** indicating how far you've leveraged the tech
2. **Focus on implementation** over ideas in Wave 6
3. **Deep integration > polished frontend** (but both matter)
4. **Be honest** about current state - judges read the code
5. **Ask questions** in dev-chat or AMAs before submission

---

*Document generated from Linera Discord scrape - January 22, 2026*
