import { get } from 'svelte/store';
import { effectiveDirectoryId, isProduction } from './stores/directoryStore';
import { env } from './stores/authStore';

let BASE_URL;
if (import.meta.env.BASE_URL && import.meta.env.BASE_URL !== '/') {
  BASE_URL = import.meta.env.BASE_URL;
} else {
  BASE_URL = "http://localhost:8000";
}

// pull env variables from .env
const API_URL = import.meta.env.API_URL || `${BASE_URL}/api`;

// Add a new function to refresh the token
async function refreshToken() {
  const response = await fetch(`${BASE_URL}/refresh-token`, {
    method: 'POST',
    headers: getAuthHeaders(),
  });

  if (!response.ok) {
    throw new Error('Failed to refresh token');
  }

  const data = await response.json();
  localStorage.setItem('authToken', data.token);
  return data.token;
}

// Modify the getAuthHeaders function to use the refreshed token
function getAuthHeaders() {
  const token = localStorage.getItem('authToken');
  return {
    'Authorization': `Bearer ${token}`,
    'Content-Type': 'application/json',
  };
}

// Create a wrapper function for API calls that handles token refreshing
async function apiCall(url, options = {}) {
  options.headers = { ...options.headers, ...getAuthHeaders() };
  
  try {
    let response = await fetch(url, options);

    if (response.status === 401) {
      // Token might be expired, try to refresh it
      const newToken = await refreshToken();
      
      // Update the Authorization header with the new token
      options.headers['Authorization'] = `Bearer ${newToken}`;
      
      // Retry the original request with the new token
      response = await fetch(url, options);
    }

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return response.json();
  } catch (error) {
    console.error('API call failed:', error);
    throw error;
  }
}

export async function fetchDashboardStats() {
  // TODO: Remove this condition once real API is implemented
  if (env === 'production') {
    return apiCall(`${API_URL}/admin/dashboard-stats`);
  } else {
    // TODO: Remove this else block once real API is implemented
    console.log('Using fake dashboard stats for non-production environment');
    await new Promise(resolve => setTimeout(resolve, 500)); // Simulate API delay
  
    // TODO: Remove this mock data once real API is implemented
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
}

// Update existing API functions to use the new apiCall wrapper
export async function fetchListings() {
  const directoryId = get(effectiveDirectoryId);
  if (!directoryId) {
    throw new Error("No directory selected");
  }
  return apiCall(`${API_URL}/listings?directory_id=${directoryId}`);
}

export async function searchListings(query) {
  const directoryId = get(effectiveDirectoryId);
  if (!directoryId) {
    throw new Error("No directory selected");
  }
  return apiCall(`${API_URL}/listings/search?q=${query}&directory_id=${directoryId}`);
}

export async function fetchAdPurchases() {
  return apiCall(`${API_URL}/admin/ad-purchases`);
}

export async function fetchListingById(id) {
  return apiCall(`${API_URL}/admin/listings/${id}`);
}

export async function fetchUsers() {
  return apiCall(`${API_URL}/admin/users`);
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