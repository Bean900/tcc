# Traveling Cook Calculator

## Introduction

The traveling cook calculator (TCC) is built to simplify the planning and execution of Cook & Run, Running Dinner, and similar multi-stage dining events. Its primary goal is to calculate team assignments and routes as efficiently as possible while minimizing organizational overhead.

A strong focus has been placed on data privacy, flexibility, and ease of use. Organizers can choose between a fully local workflow or enhanced collaboration features via an optional cloud setup. Participants can enter their own data, routes are calculated automatically, and organizers retain full control over validation and execution.

In short, the application reduces manual coordination effort, avoids error-prone spreadsheets, and provides a privacy-conscious alternative to centralized event-planning platforms.

## Features

Data protection is a core design principle of this project.
* All participant and event data can be entered, stored, and processed entirely locally.
* No data is transferred to external services unless cloud functionality is explicitly enabled or OpenStreetMap is used.

When cloud features are used:
* Only the data required for collaboration and synchronization is stored.
* The system is designed to allow self-hosting, ensuring that organizers remain in full control of where participant data is stored.
* Participant-provided data (such as addresses) is used exclusively for event planning and route calculation.
* Data is never sold, shared with third parties, or used for analytics beyond the scope of the event.

## Quick Start
The fastest way to get started is to use the hosted version of the application:

1. Visit https://XYZ
1. Create a new project
1. Configure your event settings
1. Share the generated link or QR code with participants

No local installation is required for this workflow. All core features are available directly through the browser.

## Installation

### Client
Coming soon

### Server
Coming soon


## Contribute

### General Setup
To install, run, and develop the client locally, the following steps are required:

1. Install [Git](https://git-scm.com/install/)
1. Install [Rust](https://rust-lang.org/tools/install/)
1. Clone the repository:
   ```bash
   git clone https://github.com/Bean900/tcc.git
   cd tcc
   ```
   
### Setup Client
1. Go to the client folder:
   ```bash
   cd client
   ```
1. Install [Dioxus](https://dioxuslabs.com/learn/0.7/getting_started/)
1. Install [Node.js](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
1. Install Node.js dependencies (required for Tailwind CSS):
   ```bash
   npm install
   ```
   or, if using pnpm:
   ```bash
   pnpm install
   ```
1. Install Rust dependencies:
   ```bash
   cargo fetch
   ```
If no errors occur, the client is ready to started and can be further developed.

For local testing, the following steps must then be followed:

1. Start Tailwind in watch mode:
   ```bash
   npx @tailwindcss/cli -i ./assets/input.css -o ./assets/output.css --watch
   ```
2. In a second terminal, start the Dioxus development server:
   ```bash
   dx serve --platform web
   ```

This will start the application in development mode with hot reloading enabled for both Rust and CSS changes.


To build the application:
```bash
dx bundle --web --release
```


### Setup Server

#### Setup Database
1. Install Docker

2. start postgres with docker 
```bash
docker run --name tcc-postgres -e POSTGRES_PASSWORD=mysecretpassword -d -p 5432:5432 postgres
```
3. Export connection string to DATABASE_URL. if not the default postgresql://postgres:mysecretpassword@localhost:5432/postgres is set

To build the application:
```bash
cargo build
```

Install sudo apt install libpq5

### Setup 0auth
1. Create account account at auth0.com
2. fill .env file


### Setup code
1. Fill .env file
2. cd into server folder
3. install libpq. Ubuntu -> sudo apt install libpq-dev
3. Run `cargo build`
4. run tcc server `cargo run`
5. run tests on tcc server `cargo test`