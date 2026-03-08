# WRF (Warframe Relic Finder)

WRF is a real-time matchmaking tool for Warframe Relic runs. It aims to helps players find groups for specific relics without the constant copy-pasting messages into recruit chat

## Features

### Real-time

Players can create or join lobbies for relics. State changes like players joining or leaving are pushed to all clients immediately

### Identity Verification

Users can verify their Warframe ID to get a cool check mark and for people to see that you are actually using your real in-game name

### Filtering

Filter lobbies by relic type, refinement (Radiant, Flawless, etc.), region, and/or rotation

## Tech Stack

- **Backend:** SpacetimeDB (Rust)
- **Frontend:** SvelteKit 5, Skeleton UI, Tailwind CSS
- **Auth:** Clerk

## Getting Started

### Prerequisites

- Rust and Node.js (v20+)
- SpacetimeDB CLI
- `just` (optional runner)

### Setup

1. **Backend**
   Start the local SpacetimeDB server:

    ```bash
    just dev
    ```

    In another terminal, publish the module:

    ```bash
    just publish
    ```

    This will also generate the TypeScript bindings for the frontend.

2. **Frontend**
   Install dependencies and set up environment variables:
    ```bash
    cd frontend
    npm install
    cp .env.example .env.development
    ```
    Start the dev server:
    ```bash
    npm run dev
    ```

## Development

For local testing with Clerk, you can use `test+clerk_test@example.com` with the verification code `424242`. See [`ACCOUNT_MOCKING.md`](/ACCOUNT_MOCKING.md) for more details.

## Project Structure

- `/spacetimedb`: Rust backend module (logic, tables, and reducers).
- `/frontend`: SvelteKit application and UI.
- `/assets`: Design assets and mockups.
- `justfile`: Commands for common development tasks.
