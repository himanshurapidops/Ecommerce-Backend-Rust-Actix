<!--  -->
<div align="center">

# Ecommerce Backend with Rust and Actix

[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.67+-orange.svg)](https://www.rust-lang.org)
[![Actix Web](https://img.shields.io/badge/Actix%20Web-4-green)](https://actix.rs/)

</div>

This repository contains the backend implementation of an e-commerce application built using Rust and the Actix Web framework. It provides a RESTful API for managing products, users, carts, orders, and addresses, with JWT authentication and PostgreSQL as the database.


```
# 🦀 Rust Microservices Starter Kit

A production-ready Rust microservice architecture with:

- 🚀 High-performance APIs using **Actix Web**
- 🐓 Distributed SQL with **CockroachDB**
- 📩 Asynchronous messaging with **NATS (JetStream)**
- 🐳 Full **Docker Compose** setup for easy development and deployment

---

## ✨ Features

- Multi-container orchestration via Docker Compose
- Two Rust-based services:
  - `rust-server`: Main Actix Web backend
  - `email-service`: NATS-based async worker
- CockroachDB with automatic SQL migration
- NATS with JetStream enabled for messaging
- Health checks for core services
- Easy `.env`-based configuration
- Clean, multi-stage Docker builds
- Easily extensible and production ready

---

## Setup Instructions

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/your-username/ecommerce-platform.git
   cd ecommerce-platform
   ```

2. **Run the Application**: Build and start all services using Docker Compose:

   ```bash
   docker-compose up --build
   ```

3. **Access the Services**:

   - Web Server: `http://localhost:4000`
   - Email Service: `http://localhost:5000`
   - CockroachDB Admin UI: `http://localhost:8080`
   - NATS Monitoring: `http://localhost:8222`

4. **Stop the Services**:

   ```bash
   docker-compose down
   ```