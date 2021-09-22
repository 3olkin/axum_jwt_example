# Axum JWT example

This repository provides an example of:

- Axum REST API
- Axum GraphQL API
- Axum CORS config
- Error handling
- JWT authentication
- Interaction with the database
- Password encryption
- Payload validation

## Required

- Rust
- Docker and docker-compose or Postgresql server

## Recommended

- [Diesel CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) (for database migrations)

## Usage

- cp .env.example .env
- docker-compose up -d
- diesel migration run
- cargo run --release

## Api

- POST `/register` - required fields: `name, email, password`, returns bearer token
- POST `/login` - required fields: `email, password`, returns bearer token
- GET `/authorize` - returns user
- GET `/graphql` - graphql playground
- POST `/graphql` - graphql endpoint
