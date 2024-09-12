// pull env variables from .env
const API_URL = import.meta.env.API_URL || "http://localhost:8000/api";

console.log("API_URL:", API_URL);

export async function fetchBusinesses() {
  console.log("Fetching businesses from:", `${API_URL}/businesses`);
  const response = await fetch(`${API_URL}/businesses`);
  console.log("Response:", response);
  if (!response.ok) {
    throw new Error("Failed to fetch businesses");
  }
  return response.json();
}

export async function searchBusinesses(query) {
  console.log("Searching businesses with query:", query);
  const response = await fetch(
    `${API_URL}/businesses/search?q=${encodeURIComponent(query)}`,
  );
  console.log("Response:", response);
  if (!response.ok) {
    throw new Error("Failed to search businesses");
  }
  return response.json();
}

export async function fetchBusinessById(id) {
  console.log("Fetching business details for id:", id);
  const response = await fetch(`${API_URL}/businesses/${id}`);
  console.log("Response:", response);
  if (!response.ok) {
    throw new Error("Failed to fetch business details");
  }
  return response.json();
}

export async function loginUser(credentials) {
  console.log("Logging in user");
  const response = await fetch(`${API_URL}/users/login`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(credentials),
  });
  if (!response.ok) {
    const error = new Error('Failed to login');
    error.status = response.status;
    throw error;
  }
  return response.json();
}
export async function registerUser(userData) {
  console.log("Registering user");
  const response = await fetch(`${API_URL}/users/register`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(userData),
  });
  if (!response.ok) {
    const error = new Error('Failed to register user');
    error.status = response.status;
    throw error;
  }
  return response.json();
}