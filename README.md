# CPressLink

**A full-stack link shortening web app UI** — built as a single self-contained HTML file with no external dependencies beyond Google Fonts.

---

## Overview

CPressLink is a feature-complete front-end prototype for a URL shortening platform. It covers every screen a real product would need: a public landing page, an authenticated dashboard, a campaign analytics view with a link switcher, and a full account settings panel — all navigable from a single persistent top nav.

---

## Pages

### Home (Landing)
- Large serif hero headline with animated fade-in
- Live URL shortener input — paste any URL and get a `cpress.link/xxxxx` slug instantly
- Copy button appears on the result strip with a toast confirmation
- Feature grid (6 cards): Instant Shortening, Deep Analytics, Smart Routing, Link Management, 99.98% Uptime, API Access
- How It Works — 3-step flow with dashed connector line
- Recent Links table with copy buttons on each row
- CTA banner linking to Dashboard and Analytics

### Dashboard
- **Loading skeleton** — animated shimmer placeholder shown for ~1.8s on every visit, matching the "Loading Dashboard…" state from the design references
- **Architecture Lead Dashboard** layout with 3 stat cards: Total Clicks, Avg. CTR, Top Origin (matches image 4 from the reference set)
- Shorten bar — paste a URL, generate a slug, copy it with one click
- Network Traffic bar chart (Mon–Today)
- Active Short Links table with destination preview, mini sparkline activity chart, click count, and per-row copy button
- Pagination controls (Previous / 1 2 3 / Next)
- **Architectural Pulse** dark bar — live system status showing Active Nodes, Structure Integrity, and Global Render Speed

### Analytics
- **Link Switcher sidebar** — lists all 7 links with name, slug, and click count; click any to switch the entire detail view
- Search/filter input to find links by name or slug
- ← → arrow buttons to step through links sequentially
- Per-link detail pane:
  - Status eyebrow + campaign title
  - Short link and destination pills with copy/open buttons
  - 3 stat cards: Total Clicks, Avg. CTR, Top Source
  - Smooth cubic bezier line chart (30-day clicks over time) — redraws per link
  - Top Referrers with animated progress bars
  - Device Breakdown donut chart (Mobile / Desktop / Tablet)
  - Geographical Reach with top 3 countries

### Settings
- **Sidebar navigation** with 4 tabs — each switches content without a page reload:
  - **Profile** — Full Name, Email, Username, Website, Bio; accent colour picker (5 swatches: Forest, Ocean, Ember, Violet, Slate) that updates the entire app's colour scheme live
  - **Security** — Reset Password, Two-Factor Auth, API key management (Production + Test keys with Regenerate), Active Sessions list with Revoke buttons
  - **Preferences** — 5 toggleable settings (Email Notifications, Daily Reports, Dark Mode, Link Expiry Alerts, Click Milestones); link defaults (slug length, expiry)
  - **Billing** — Pro Plan card with dark gradient, usage bars (links, API requests, custom domains), invoice history with PDF download links

---

## Features

| Feature | Detail |
|---|---|
| Copy to clipboard | Works on every link — landing result, dashboard result, table rows, analytics pills |
| Toast notification | Slides up from bottom confirming every copy action |
| Dashboard skeleton | Shimmer animation on every dashboard visit, transitions to real content |
| Analytics link switcher | 7 links, each with unique chart shape, referrers, devices, and geo data |
| Live chart rendering | SVG line chart redraws with smooth cubic bezier curves per link |
| Donut chart | SVG stroke-dasharray donut updates per link's device split |
| Accent colour theming | Settings → Profile → colour swatch updates `--accent` and `--dark` CSS variables globally |
| Settings tabs | Profile / Security / Preferences / Billing — all wired up, no page reload |
| Pagination UI | Dashboard links table has Previous / 1 2 3 / Next controls |
| Sparklines | Mini SVG trend lines in the dashboard links table per row |

---

## Files

```
cpresslink-dashboard.html   Main app — all 4 pages in one file
cpresslink-hero.html        Original standalone hero section (early prototype)
README.md                   This file
```

---

## Design System

| Token | Value |
|---|---|
| Background | `#f2f5f3` — soft sage white |
| Card | `#ffffff` |
| Dark / Primary | `#1a2e23` — deep forest green |
| Accent | `#2d5a3d` — mid forest green |
| Accent Light | `#e8f0ea` — pale green tint |
| Muted | `#7a9085` |
| Border | `rgba(45,90,61,0.10)` |
| Border Radius | `12px` |
| Shadow | `0 2px 16px rgba(26,46,35,0.07)` |
| Display Font | Instrument Serif (Google Fonts) |
| Body Font | DM Sans (Google Fonts) |

---

## How to Run

No build step. No dependencies. Open `cpresslink-dashboard.html` directly in any modern browser.

```bash
open cpresslink-dashboard.html
# or just drag-drop into Chrome / Firefox / Safari
```

---

## Reference Screenshots

The UI was designed to match a series of reference screens from a "LinkArch" design system:

- **Image 1** — Architectural Dashboard with Network Traffic chart, Active Links table, and Pulse bar
- **Image 2** — Loading/skeleton state with shimmer placeholders
- **Image 3** — Settings / Account Architecture with Profile form and Preferences toggles
- **Image 4** — Architecture Lead Dashboard with Total Clicks, Avg. CTR, and Top Origin stat cards
- **Image 5** — Summer-Campaign-2024 analytics detail with line chart, referrers, device donut, and geo breakdown

---

## Interaction Map

```
Home
 ├── "Start Shortening Free" → Dashboard
 ├── "View Analytics Demo →" → Analytics
 ├── URL input → shorten → copy
 ├── Recent links table → copy per row
 └── CTA banner → Dashboard / Analytics

Dashboard
 ├── Loads with skeleton → fades to real content
 ├── Shorten bar → generate slug → copy
 ├── "Full Report →" → Analytics
 └── Links table → copy per row

Analytics
 ├── Sidebar: click any link → switches all detail
 ├── Search input → filters sidebar list
 ├── ← → buttons → step through links
 └── Short link pill copy button → clipboard

Settings
 ├── Profile tab → edit form + accent colour picker
 ├── Security tab → password, 2FA, API keys, sessions
 ├── Preferences tab → toggles + link defaults
 └── Billing tab → plan card, usage bars, invoices
```

---

*© 2026 CPressLink. Built with Architectural Precision.*
