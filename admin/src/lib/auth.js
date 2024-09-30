import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import { isAuthenticated } from './stores/authStore';
import { api } from './api';

export async function checkAuth() {
  if (browser) {
    const token = localStorage.getItem('authToken');
    console.log("Checking auth", token);
    if (token) {
      console.log("Token found, verifying session");
      try {
        // Verify the token and get user session info
        const sessionInfo = await api.user.verifySession();
        if (sessionInfo.isValid) {
          isAuthenticated.set(true);
          // we can store additional user info if needed
          // For example: userStore.set(sessionInfo.user);
          return true;
        } else {
          // Token is invalid or expired
          logout();
          return false;
        }
      } catch (error) {
        console.error('Error verifying session:', error);
        logout();
        return false;
      }
    } else {
      console.log("No token found, setting auth to false");
      isAuthenticated.set(false);
      return false;
    }
  }
  return false;
}

export function login(token) {
  if (browser) {
    localStorage.setItem('authToken', token);
    isAuthenticated.set(true);
  }
}

export function logout() {
  if (browser) {
    localStorage.removeItem('authToken');
    isAuthenticated.set(false);
    // Clear any other user-related data from stores if needed
    // For example: userStore.set(null);
  }
}

// Update this function in your API file
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
  const data = await response.json();
  login(data.token); // Store the token
  return data;
}
