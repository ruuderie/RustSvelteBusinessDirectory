# Business Directory

This project is a business directory website using Svelte for the frontend and Rust for the backend API. It allows users to view and search for business listings, as well as register and login.

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
- User registration and login

## API Endpoints

### Businesses
- GET `/api/businesses`: Fetch all businesses
- GET `/api/businesses/search?q={query}`: Search for businesses
- GET `/api/businesses/{id}`: Fetch a specific business by ID

### Users
- POST `/api/users/register`: Register a new user
- POST `/api/users/login`: Login a user

## Database

The project uses PostgreSQL as the database. Make sure to set up the database and update the `.env` file in the `backend` directory with the correct database credentials.

## Development

To work on this project, you'll need to have Node.js, npm, and Rust installed on your system. Follow the instructions in the respective directories to set up and run the frontend and backend components.

### Environment Variables

Make sure to set up the following environment variables:

- Frontend:
  - `API_URL`: The URL of the backend API (default: "http://localhost:8000/api")

- Backend:
  - Database connection string (in the `.env` file)

## Security Note

This project includes user authentication. Ensure that you're using HTTPS in production and following best practices for handling user data and authentication tokens.
