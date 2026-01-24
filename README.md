# Project Overview

## Introduction

The traveling cook calculator (TCC) is designed to support the planning and organization of so-called *Cook & Run*, *Running Dinner*, or similar multi-stage dining and running events. From the very beginning, development has been driven by a strong focus on data privacy and by the goal of calculating routes and team allocations as optimally as possible.

At its core, the application allows all data entry and all calculations to be performed completely locally. This enables event organizers to plan and execute events without transmitting any personal or location-based data to external systems.

For users who require additional functionality, the application also offers an optional cloud mode. Even in this scenario, data protection remains a top priority: the cloud can be self-hosted, allowing full control over where and how data is stored. Instructions on how to set up and operate your own cloud instance are provided in a later chapter.

When cloud functionality is enabled, projects can be accessed and managed from multiple devices, such as different computers or mobile phones. In addition, organizers can generate a shareable link or QR code through which participating teams can directly enter their information and subsequently access their "walking" sheets. This significantly reduces manual effort, as the organizer only needs to review and validate the submitted data instead of collecting and entering it manually.

## Features

* Privacy-first architecture with full local-only operation
* Optimized route and team calculation for Running Dinner–style events
* Optional cloud support with self-hosting capability
* Cross-device project access (desktop and mobile)
* Shareable links and QR codes for team self-registration
* Central validation and management of all submitted data

## Installation (Client)

### Prerequisites

To install, run, and develop the client locally, the following tools are required:

- **Rust** (stable toolchain)
  - Install via rustup: https://rustup.rs
- **Cargo** (comes with Rust)
- **Node.js** (LTS version recommended)
- **npm** or **pnpm** (for frontend tooling)
- **Git**

The client is built using **Rust**, **Dioxus**, and **Tailwind CSS**. Tailwind is integrated via the Node.js toolchain and is required both for development and for production builds.

---

### Client Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd <project-directory>
   ```

2. Install Rust dependencies:
   ```bash
   cargo fetch
   ```

3. Install Node.js dependencies (required for Tailwind CSS):
   ```bash
   npm install
   ```
   or, if using pnpm:
   ```bash
   pnpm install
   ```


If no errors occur, the client is ready to be started.

---

## Development Setup

For active development and implementation work, the following processes must be running:

- The Dioxus development server
- The Tailwind CSS watcher

### Start Development Mode

1. Start Tailwind in watch mode:
   ```bash
   npx @tailwindcss/cli -i ./assets/input.css -o ./assets/output.css --watch
   ```

2. In a second terminal, start the Dioxus development server:
   ```bash
   dx serve --platform web
   ```

This will start the application in development mode with hot reloading enabled for both Rust and CSS changes.

---

## Starting the Client

### Development Mode

For local development:

```bash
dx serve --platform web
```

The application will be available at the local address printed in the terminal (typically `http://localhost:8080`).

### Production Build

To create an optimized production build:

1. Build Tailwind CSS:
   ```bash
   npm run build:css
   ```

2. Build the Dioxus application:
   ```bash
   dx build --release
   ```

The resulting artifacts can be found in the build output directory and can be deployed or packaged depending on the target platform.

---

## Further Development

### Recommended Workflow

- Keep Tailwind running in watch mode during development
- Use `dx serve` for fast feedback and hot reloads
- Run `cargo fmt` and `cargo clippy` regularly to maintain code quality

### Useful Commands

```bash
cargo fmt        # Format Rust code
cargo clippy    # Lint Rust code
cargo test      # Run tests
```

Additional chapters will cover application architecture, cloud setup, and advanced configuration options.

