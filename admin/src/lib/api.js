import { get } from 'svelte/store';
import { effectiveDirectoryId, isProduction } from './stores/directoryStore';

// pull env variables from .env
const API_URL = import.meta.env.API_URL || "http://localhost:8000/api";

console.log("API_URL:", API_URL);

// File: src/lib/api.js

function getAuthHeaders() {
  const token = localStorage.getItem('authToken');
  return {
    'Authorization': `Bearer ${token}`,
    'Content-Type': 'application/json',
  };
}

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

  // Return mock data for a successful company
  return {
    totalUsers: 150000,
    activeListings: 75000,
    adPurchases: 12000,
    revenue: 1800000,
    totalCategories: 500,
    monthlyRevenue: [1500000, 1550000, 1600000, 1650000, 1700000, 1750000, 1800000],
    userGrowth: [120000, 125000, 130000, 135000, 140000, 145000, 150000],
    listingGrowth: [60000, 62500, 65000, 67500, 70000, 72500, 75000],
    adSalesGrowth: [9000, 9500, 10000, 10500, 11000, 11500, 12000]
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
  const response = await fetch(`${API_URL}/listings?directory_id=${directoryId}`, {
    headers: getAuthHeaders(),
  });
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
  const response = await fetch(`${API_URL}/listings/search?q=${query}&directory_id=${directoryId}`, {
    headers: getAuthHeaders(),
  });
  console.log("Response:", response);
  if (!response.ok) {
    throw new Error("Failed to search listings");
  }
  return response.json();
}

export async function fetchAdPurchases() {
  console.log("Fetching ad purchases");
  const response = await fetch(`${API_URL}/ad-purchases`, {
    headers: getAuthHeaders(),
  });
  if (!response.ok) {
    throw new Error("Failed to fetch ad purchases");
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