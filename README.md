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

Before installing the client, ensure that the following tools are available on your system:

* A modern operating system (Windows, macOS, or Linux)
* A recent web browser (for web-based usage)
* Git (for development and contribution)
* The required runtime environment as described below

Depending on your setup, additional tools may be required for development work (see *Development Setup*).

### Client Installation

1. Clone the repository:

   ```bash
   git clone git@github.com:Bean900/tcc.git
   cd tcc
   ```

2. Install the required dependencies:

   ```bash
   <dependency-install-command>
   ```

3. Verify that the installation completed successfully by running the version or help command:

   ```bash
   <client-command> --help
   ```

## Development Setup

For implementation and development work, the following components must be running:

* The client application in development mode
* Any required local services (e.g. local storage, optional backend services)

Typical development steps:

1. Install all development dependencies:

   ```bash
   <development-dependency-install-command>
   ```

2. Start the development environment:

   ```bash
   <development-start-command>
   ```

3. Make code changes and verify them using the available build or test commands.

## Starting the Client

To start the client in normal (non-development) mode, run:

```bash
<client-start-command>
```

Once started, the client will be accessible via the configured interface (for example, a local desktop window or a browser at a local address).

Further configuration options, advanced features, and cloud setup instructions are described in the following chapters of this documentation.
