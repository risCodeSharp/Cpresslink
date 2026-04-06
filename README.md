# CPressLink

🔗 **Live Demo:** https://cpresslink.netlify.app/

**A modern link shortening platform UI** — evolving into a full-stack system powered by **Vue + Axum (Rust)**.

---

## Overview

CPressLink is a production-style URL shortening platform designed with a strong focus on **architecture, scalability, and UX clarity**.

It began as a **single-file prototype**, and is now being refactored into a **modular Vue frontend**, with plans to integrate a high-performance backend using **Axum (Rust)**.

---

## Tech Stack

### Frontend

* Vue 3 + TypeScript
* Vue Router
* Feature-based architecture
* SVG-based charts (custom)

### Backend *(Planned)*

* Rust
* Axum (web framework)
* REST / JSON APIs
* Scalable service architecture

---

## Features

### Core Functionality

* URL shortening with instant slug generation
* Copy-to-clipboard across the app
* Toast notification system

### Dashboard

* Skeleton loading state
* Stats overview (Clicks, CTR, Top Origin)
* Traffic visualization
* Active links table with sparklines

### Analytics

* Link switcher system
* Dynamic charts (30-day trends)
* Referrer tracking
* Device breakdown (donut chart)
* Geo analytics

### Settings System

* Profile management + live theme switching
* Security (password, 2FA, API keys)
* Preferences (notifications, defaults)
* Billing UI (plans, usage, invoices)

---

## Project Structure

```bash
frontend/
└── src/
    ├── assets/            # Global styles, fonts, images
    ├── components/        # Reusable UI components

    ├── features/          # Feature-based modules
    │   ├── analytics/
    │   ├── dashboard/
    │   └── settings/

    ├── views/             # Route-level pages
    │   ├── Home.vue
    │   ├── Dashboard.vue
    │   ├── Analytics.vue
    │   ├── SettingsLayout.vue
    │   └── Settings/
    │       ├── Profile.vue
    │       ├── Security.vue
    │       ├── Preferences.vue
    │       ├── BillingPlans.vue
    │       └── API&Integrations.vue

    ├── router/            # Routing config
    ├── stores/            # State management
    ├── App.vue
    ├── main.ts
    ├── types.ts
    └── env.d.ts
```

---

## Architecture Approach

* **Feature-first design** for scalability
* **Separation of concerns** (views vs logic vs UI)
* **Reusable components system**
* **Extensible settings architecture**
* Backend planned with **Axum for performance and safety**

---

## Roadmap

### Phase 1 (Completed)

* UI prototype (single HTML file)

### Phase 2 (Current)

* Vue modular frontend
* Feature-based architecture

### Phase 3 (Upcoming)

* Axum backend integration
* Real URL shortening service
* Database integration
* Authentication system

### Phase 4 (Future)

* Custom domains
* Team workspaces
* Advanced analytics

---

## How to Run

```bash
npm install
npm run dev
```

---

## Vision

CPressLink is being built as a **portfolio-grade system design project** that demonstrates:

* Frontend architecture at scale
* Backend performance using Rust
* Real-world SaaS design patterns

---

## License

MIT

---

*© 2026 CPressLink — Built with Architectural Precision.*
