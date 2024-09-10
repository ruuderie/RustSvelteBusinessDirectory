# Business Directory

This project is a business directory website using Svelte for the frontend and Rust for the backend API. It allows users to view and search for business listings.

## Project Structure

The project is divided into two main parts:

1. Frontend (Svelte with SvelteKit)
2. Backend (Rust with Axum and SeaORM)

### Frontend

The frontend is located in the `frontend` directory. It's built using Svelte and SvelteKit for routing.

To run the frontend:

1. Navigate to the `frontend` directory
2. Install dependencies: `npm install`
3. Start the development server: `npm run dev`

The frontend will be available at `http://localhost:5000`.

### Backend

The backend is located in the `backend` directory. It's built using Rust with the Axum framework and SeaORM for database operations.

To run the backend:

1. Navigate to the `backend` directory
2. Build and run the project: `cargo run`

The backend API will be available at `http://localhost:8000`.

## Features

- Display a list of businesses
- Search for businesses by name or category
- View detailed information about a specific business

## API Endpoints

- GET `/api/businesses`: Fetch all businesses
- GET `/api/businesses/search?q={query}`: Search for businesses
- GET `/api/businesses/{id}`: Fetch a specific business by ID

## Database

The project uses PostgreSQL as the database. Make sure to set up the database and update the `.env` file in the `backend` directory with the correct database credentials.

## Development

To work on this project, you'll need to have Node.js, npm, and Rust installed on your system. Follow the instructions in the respective directories to set up and run the frontend and backend components.
