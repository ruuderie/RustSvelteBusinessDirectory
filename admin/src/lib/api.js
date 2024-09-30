import { get } from 'svelte/store';
import { effectiveDirectoryId, isProduction } from './stores/directoryStore';
import { env } from './stores/authStore';

let BASE_URL;
if (import.meta.env.BASE_URL && import.meta.env.BASE_URL !== '/') {
  BASE_URL = import.meta.env.BASE_URL;
} else {
  BASE_URL = "http://localhost:8000";
}

const API_URL = import.meta.env.API_URL || `${BASE_URL}/api`;

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

function getAuthHeaders() {
  const token = localStorage.getItem('authToken');
  return {
    'Authorization': `Bearer ${token}`,
    'Content-Type': 'application/json',
  };
}

async function apiCall(resource, endpoint, options = {}) {
  const url = `${API_URL}/${resource}${endpoint}`;
  options.headers = { ...options.headers, ...getAuthHeaders() };
  
  try {
    let response = await fetch(url, options);

    if (response.status === 401) {
      const newToken = await refreshToken();
      options.headers['Authorization'] = `Bearer ${newToken}`;
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

const userApi = {
  login: (credentials) => {
    console.log("Attempting to log in user:", credentials.email);
    console.log("Login URL:", `${BASE_URL}/login`);
    return fetch(`${BASE_URL}/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(credentials),
    }).then(response => {
      console.log("Login response status:", response.status);
      if (!response.ok) {
        return response.json().then(errorData => {
          console.error("Login error:", errorData);
          const error = new Error(errorData.message || 'Failed to login');
          error.status = response.status;
          error.details = errorData.errors;
          throw error;
        });
      }
      return response.json().then(data => {
        console.log("Login successful, received data:", data);
        return data;
      });
    });
  },
  register: (userData) => {
    console.log("Registering user");
    return fetch(`${BASE_URL}/register`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(userData),
    }).then(response => {
      if (!response.ok) {
        const error = new Error('Failed to register user');
        error.status = response.status;
        throw error;
      }
      return response.json();
    });
  },
  logout: () => apiCall('users', '/logout', { method: 'POST' }),
  getProfile: () => apiCall('users', '/profile'),
  updateProfile: (profileData) => apiCall('users', '/profile', { 
    method: 'PUT', 
    body: JSON.stringify(profileData) 
  }),
};

const listingApi = {
  fetchListings: () => {
    const directoryId = get(effectiveDirectoryId);
    if (!directoryId) {
      throw new Error("No directory selected");
    }
    return apiCall('listings', `?directory_id=${directoryId}`);
  },
  searchListings: (query) => {
    const directoryId = get(effectiveDirectoryId);
    if (!directoryId) {
      throw new Error("No directory selected");
    }
    return apiCall('listings', `/search?q=${query}&directory_id=${directoryId}`);
  },
  fetchListingById: (id) => apiCall('listings', `/${id}`),
};

const adminApi = {
  fetchDashboardStats: () => {
    if (env === 'production') {
      return apiCall('admin', '/dashboard-stats');
    } else {
      console.log('Using fake dashboard stats for non-production environment');
      return new Promise(resolve => {
        setTimeout(() => {
          resolve({
            totalUsers: 150000,
            activeListings: 75000,
            adPurchases: 12000,
            revenue: 1800000,
            totalCategories: 500,
            monthlyRevenue: [500000, 750000, 800000, 1250000, 1400000, 1750000, 2050000],
            userGrowth: [60000, 80000, 94250, 101250, 115741, 135741, 168521],
            listingGrowth: [60000, 62500, 65000, 67500, 70000, 72500, 75000],
            adSalesGrowth: [9000, 9500, 10000, 10500, 11000, 11500, 12000]
          });
        }, 500);
      });
    }
  },
  fetchAdPurchases: () => apiCall('admin', '/ad-purchases'),
  fetchUsers: () => apiCall('admin', '/users'),
  fetchDirectories: () => apiCall('admin', '/directories'), // Add this line
};

export const api = {
  user: userApi,
  listing: listingApi,
  admin: adminApi,
};