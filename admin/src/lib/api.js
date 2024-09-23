import { get } from 'svelte/store';
import { effectiveDirectoryId, isProduction } from './stores/directoryStore';

// pull env variables from .env
const API_URL = import.meta.env.API_URL || "http://localhost:8000/api";

console.log("API_URL:", API_URL);

// File: src/lib/api.js

export async function loginUserMock({ email, password }) {
  // Simulate API call
  await new Promise(resolve => setTimeout(resolve, 1000));
  
  if (email === 'admin@oply.co' && password === 'password') {
    return 'mock-jwt-token';
  } else {
    throw new Error('Invalid credentials');
  }
}

export async function fetchDashboardStats() {
  // Simulate API call
  await new Promise(resolve => setTimeout(resolve, 1000));
  
  // Check if user is authenticated (you might want to use a more robust check in a real app)
  const token = localStorage.getItem('authToken');
  if (!token) {
    throw new Error('User is not authenticated');
  }

  // Return mock data
  return {
    totalUsers: 10567,
    activeListings: 3289,
    adPurchases: 1456,
    revenue: 287650.75,
    userGrowth: [4000, 4500, 5200, 5800, 6500, 7200, 8100],
    listingGrowth: [2400, 2800, 3200, 3600, 3900, 4200, 4500],
    revenueGrowth: [50000, 60000, 75000, 90000, 110000, 130000, 150000]
  };
}

// Add a function to get the auth token from localStorage
function getAuthToken() {
  return localStorage.getItem('authToken');
}

export async function fetchDirectories() {
  if (get(isProduction)) {
    const directoryId = get(effectiveDirectoryId);
    if (!directoryId) {
      throw new Error("No directory configured for production");
    }
    return [{ id: directoryId, name: "Production Directory" }];
  }

  const response = await fetch(`${API_URL}/directories`);
  if (!response.ok) {
    throw new Error("Failed to fetch directories");
  }
  return response.json();
}

export async function fetchListings() {
  const directoryId = get(effectiveDirectoryId);
  if (!directoryId) {
    throw new Error("No directory selected");
  }
  const response = await fetch(`${API_URL}/listings?directory_id=${directoryId}`);
  console.log("Response:", response);
  if (!response.ok) {
    throw new Error("Failed to fetch businesses");
  }
  return response.json();
}

export async function searchListings(query) {
  const directoryId = get(effectiveDirectoryId);
  if (!directoryId) {
    throw new Error("No directory selected");
  }
  const response = await fetch(`${API_URL}/listings/search?q=${query}&directory_id=${directoryId}`);
  console.log("Response:", response);
  if (!response.ok) {
    throw new Error("Failed to search listings");
  }
  return response.json();
}

export async function fetchListingById(id) {
  const response = await fetch(`http://localhost:8000/api/listing/${id}`, {
    credentials: 'include',
  });
  
  console.log('Response:', response);
  console.log('Response headers:', response.headers);
  
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  
  const text = await response.text();
  
  let data;
  try {
    data = JSON.parse(text);
  } catch (e) {
    console.error('Error parsing JSON:', e);
    throw new Error('Invalid JSON in response');
  }
  return data;
}

export async function loginUser(credentials) {
  console.log("Logging in user");
  const response = await fetch(`${API_URL}/login`, {
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
  const response = await fetch(`${API_URL}/register`, {
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