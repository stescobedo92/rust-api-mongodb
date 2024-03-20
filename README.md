# rust-api-mongodb
This project provides a simple API for managing users using Actix web framework and MongoDB.

# Dependencies
- Actix web
- MongoDB

## Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/your_username/your_repository.git
   ```
2. Install dependencies:
    ```bash
    cd your_repository
    cargo build
   ```
3. Set up MongoDB:
   - Install MongoDB on your system.
   - Configure the MongoDB connection URI in the .env file.

4. Run the server:
    ```bash
    cargo run
    ```

# Docker Compose
You can also run this application using Docker Compose. Follow these steps:

1. Install Docker and Docker Compose on your system.
2. Create a docker-compose.yml file in the root of the project with the following content:
    ```yaml
    version: '3'
    services:
      app:
        build: .
        ports:
          - "8080:8080"
        environment:
          - MONGOURI=<your_mongodb_uri>
    ```
3. Run the following command to start the application:
    ```bash
    docker-compose up -d
    ```

# Endpoints
- `POST /users`: Create a new user.
- `GET /users/{id}`: Get a user by ID.
- `PUT /users/{id}`: Update a user by ID.
- `DELETE /users/{id}`: Delete a user by ID.
- `GET /users`: Get all users.

# Usage
- To create a user, send a `POST` request to `/users` with JSON payload containing user data.
- To get a user by ID, send a `GET` request to `/users/{id}`.
- To update a user by ID, send a `PUT` request to /users/{id} with JSON payload containing updated user data.
- To delete a user by ID, send a `DELETE` request to `/users/{id}`.
- To get all users, send a `GET` request to `/users`.