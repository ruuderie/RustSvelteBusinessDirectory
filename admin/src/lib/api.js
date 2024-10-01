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
  try {
    console.log("Refreshing token");
    const response = await fetch(`${BASE_URL}/refresh-token`, {
      method: 'POST',
      headers: getAuthHeaders(),
    });

    if (!response.ok) {
      console.error("Failed to refresh token. Status:", response.status);
      return { success: false, error: `Failed to refresh token. Status: ${response.status}` };
    } else{
      console.log("Token refreshed successfully");
    }

    const data = await response.json();
    console.log('Refresh token response data:', data);

    localStorage.setItem('authToken', data.token);
    console.log('New auth token set in localStorage');

    return { success: true, token: data.token };
  } catch (error) {
    console.error('Error in refreshToken:', error);
    return { success: false, error: error.message };
  }
}

function getAuthHeaders() {
  const token = localStorage.getItem('authToken');
  return {
    'Authorization': `Bearer ${token}`,
    'Content-Type': 'application/json',
  };
}

async function apiCall(endpoint, options = {}, isPublic = false) {
  console.log("API call endpoint:", endpoint);
  console.log("API call options:", options);
  
  const url = isPublic ? `${BASE_URL}${endpoint}` : `${API_URL}${endpoint}`;
  if (!isPublic) {
    options.headers = { ...options.headers, ...getAuthHeaders() };
  }
  console.log("API call URL:", url);

  try {
    let response = await fetch(url, options);

    if (response.status === 401 && !isPublic) {
      console.log("Token expired, attempting to refresh...");
      const refreshResult = await refreshToken();
      if (refreshResult.success) {
        console.log("Token refreshed successfully, retrying original request");
        options.headers['Authorization'] = `Bearer ${refreshResult.token}`;
        response = await fetch(url, options);
      } else {
        console.error("Failed to refresh token:", refreshResult.error);
        throw new Error("Authentication failed. Please log in again.");
      }
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

async function verifySession() {
  try {
    console.log("Verifying session");
    console.log("Auth headers:", getAuthHeaders());
    console.log("BASE_URL:", BASE_URL);
    const response = await fetch(`${BASE_URL}/validate-session`, {
      method: 'GET',
      headers: getAuthHeaders(),
    });
    console.log("Response:", response);

    if (!response.ok) {
      console.error("Failed to verify session. Status:", response.status);
      return { isValid: false, error: `Failed to verify session. Status: ${response.status}` };
    }

    return { isValid: true };
  } catch (error) {
    console.error('Error in verifySession:', error);
    return { isValid: false, error: error.message };
  }
}

const userApi = {
  login: (credentials) => {
    console.log("Attempting to log in user:", credentials.email);
    return apiCall('/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(credentials),
    }, true);
  },
  register: (userData) => {
    console.log("Registering user");
    return apiCall('/register', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(userData),
    }, true);
  },
  logout: () => apiCall('/logout', { method: 'POST' }),
  getProfile: () => apiCall('/users/profile'),
  updateProfile: (profileData) => apiCall('/users/profile', { 
    method: 'PUT', 
    body: JSON.stringify(profileData) 
  }),
  verifySession: verifySession,
};

const listingApi = {
  fetchListings: () => {
    const directoryId = get(effectiveDirectoryId);
    if (!directoryId) {
      throw new Error("No directory selected");
    }
    return apiCall(`/listings?directory_id=${directoryId}`, {}, true);
  },
  searchListings: (query) => {
    const directoryId = get(effectiveDirectoryId);
    if (!directoryId) {
      throw new Error("No directory selected");
    }
    return apiCall(`/listings/search?q=${query}&directory_id=${directoryId}`, {}, true);
  },
  fetchListingById: (id) => apiCall(`/listings/${id}`, {}, true),
};

const adminApi = {
  fetchDashboardStats: () => {
    if (env === 'production') {
      return apiCall('/admin/dashboard-stats');
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
  fetchAdPurchases: () => apiCall('/admin/ad-purchases'),
  fetchUsers: () => apiCall('/admin/users'),
  fetchDirectories: () => apiCall('/admin/directories'),
  fetchUserById: (userId) => apiCall(`/admin/users/${userId}`),
  updateUser: (userId, userData) => apiCall(`/admin/users/${userId}`, {
    method: 'PUT',
    body: JSON.stringify(userData)
  }),
};

export const api = {
  user: userApi,
  listing: listingApi,
  admin: adminApi,
};