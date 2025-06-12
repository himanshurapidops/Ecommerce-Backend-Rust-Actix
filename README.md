<!--  -->
<div align="center">

# Ecommerce Backend with Rust and Actix

[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.67+-orange.svg)](https://www.rust-lang.org)
[![Actix Web](https://img.shields.io/badge/Actix%20Web-4-green)](https://actix.rs/)

</div>

This repository contains the backend implementation of an e-commerce application built using Rust and the Actix Web framework. It provides a RESTful API for managing products, users, carts, orders, and addresses, with JWT authentication and PostgreSQL as the database.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Running the Project](#running-the-project)
- [Dependencies](#dependencies)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Features

- **User Authentication:** Registration and login with JWT.
- **Product Management:** CRUD operations for products (Admin only).
- **Cart Management:** Adding, viewing, removing, and clearing cart items.
- **Order Management:** Creating orders, updating order status (Admin only), and viewing user orders.
- **Address Management:** Creating, updating, deleting, and selecting user addresses.
- **JWT Authentication:** Secure API endpoints with JWT middleware.
- **Admin Middleware:** Protected admin routes.
- **Email Notifications:** Registration and order confirmation emails via NATS.
- **Database:** PostgreSQL integration using `sqlx`.
- **Background Tasks:** Scheduled sales report generation and email sending.

## Installation

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/himanshurapidops/Ecommerce-Backend-Rust-Actix.git
    cd Ecommerce-Backend-Rust-Actix
    ```

2.  **Install Rust:**

    If you don't have Rust installed, download it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

3.  **Install PostgreSQL:**

    Make sure you have PostgreSQL installed and running. You may need to configure it according to your system.

4.  **Create a `.env` file:**

    Create a `.env` file in the root directory of the project.

    ```bash
    touch .env
    ```

    Add the following environment variables to the `.env` file, replacing the placeholder values with your actual credentials and configurations:

    ```
    DATABASE_URL=postgres://user:password@host:port/database
    JWT_SECRET=your_jwt_secret_key
    SMTP_USERNAME=your_smtp_username
    SMTP_PASSWORD=your_smtp_password
    SMTP_SERVER=smtp.gmail.com
    SMTP_PORT=587
    EMAIL_FROM=your_email@example.com
    EMAIL_TO=recipient_email@example.com
    PORT=4000
    NATS_URL=nats://localhost:4222
    ```

    **Note:** Ensure the PostgreSQL user has the necessary permissions to create tables and perform other database operations.

## Running the Project

1.  **Run Migrations:**

    ```bash
    cargo install sqlx-cli
    sqlx database create
    sqlx migrate run
    ```

2.  **Run the Backend Server:**

    ```bash
    cargo run
    ```

    This command will start the Actix Web server on the configured port (default is 4000).

## Dependencies

- **actix-web:** A powerful, pragmatic, and extremely fast web framework for Rust.
- **serde:** A framework for serializing and deserializing Rust data structures.
- **serde_json:** A crate for working with JSON data.
- **uuid:** A library for generating unique identifiers.
- **log:** A logging facade.
- **env_logger:** A logger that integrates with the `log` facade.
- **dotenvy:** A library for loading environment variables from a `.env` file.
- **sqlx:** A toolkit for working with SQL databases.
- **actix-cors:** Actix Web middleware for handling Cross-Origin Resource Sharing.
- **jsonwebtoken:** A crate for working with JSON Web Tokens.
- **tokio:** An asynchronous runtime for Rust.
- **bcrypt:** A password hashing library.
- **chrono:** Date and time library.
- **lettre:** An email library.
- **async-nats:** Asynchronous NATS client for Rust.
- **regex:** Regular expression library.
- **validator:** A library for data validation.

## Contributing

Contributions are welcome! Here's how you can contribute:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix.
3.  Implement your changes.
4.  Write tests to ensure the changes work as expected.
5.  Submit a pull request with a clear description of your changes.

## License

This project is licensed under the [MIT License](LICENSE) - see the [LICENSE](LICENSE) file for details.

## Contact

Himanshu Rapidops - [himanshuisherenow@gmail.com](mailto:himanshuisherenow@gmail.com)

```

```
