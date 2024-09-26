import { get } from 'svelte/store';
import { effectiveDirectoryId, isProduction } from './stores/directoryStore';

let BASE_URL;
if (import.meta.env.BASE_URL && import.meta.env.BASE_URL !== '/') {
  BASE_URL = import.meta.env.BASE_URL;
} else {
  BASE_URL = "http://localhost:8000";
}

// pull env variables from .env
const API_URL = import.meta.env.API_URL || `${BASE_URL}/api`;

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
    monthlyRevenue: [500000, 750000, 800000, 1250000, 1400000, 1750000, 2050000],
    userGrowth: [60000, 80000, 94250, 101250, 115741, 135741, 168521],
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
  let url = `${API_URL}/admin/ad-purchases`;
  console.log("Fetching ad purchases from:", url);
  const response = await fetch(url, {
    headers: getAuthHeaders(),
  });
  if (!response.ok) {
    throw new Error("Failed to fetch ad purchases");
  }
  return response.json();
}

export async function fetchListingById(id) {
  const response = await fetch(`${API_URL}/admin/listings/${id}`, {
    headers: getAuthHeaders(),
  });
  
  if (!response.ok) {
    const error = new Error(`HTTP error! status: ${response.status}`);
    error.status = response.status;
    throw error;
  }
  
  return response.json();
}

export async function loginUser(credentials) {
  console.log("Attempting to log in user:", credentials.email);
  console.log("Login URL:", `${BASE_URL}/login`);
  const response = await fetch(`${BASE_URL}/login`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(credentials),
  });
  
  console.log("Login response status:", response.status);
  
  if (!response.ok) {
    const errorData = await response.json();
    console.error("Login error:", errorData);
    const error = new Error(errorData.message || 'Failed to login');
    error.status = response.status;
    error.details = errorData.errors;
    throw error;
  }
  
  const data = await response.json();
  console.log("Login successful, received data:", data);
  return data;
}

export async function registerUser(userData) {
  console.log("Registering user");
  const response = await fetch(`${BASE_URL}/register`, {
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