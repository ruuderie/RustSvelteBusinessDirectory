import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import { isAuthenticated } from './stores/authStore';
import { api } from './api';
import { setUser } from './stores/userStore';
import { loadUser } from './stores/userStore';

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
          console.log("Session is valid, setting isAuthenticated to true");
          isAuthenticated.set(true);
          return true;
        } else {
          // Token is invalid or expired, try to refresh
          console.log("Session is invalid, attempting to refresh token");
          const refreshResult = await api.user.refreshToken();
          if (refreshResult.success) {
            console.log("Token refreshed successfully, setting isAuthenticated to true");
            isAuthenticated.set(true);
            return true;
          } else {
            console.error('Failed to refresh token:', refreshResult.error);
            logout();
            return false;
          }
        }
      } catch (error) {
        console.error('Error verifying session:', error);
        logout();
        return false;
      }
    } else {
      console.log("No token found, setting isAuthenticated to false");
      isAuthenticated.set(false);
      return false;
    }
  }
  return false;
}

export async function login(token, refreshToken, userData) {
  if (browser) {
    localStorage.setItem('authToken', token);
    localStorage.setItem('refreshToken', refreshToken);
    localStorage.setItem('userData', JSON.stringify(userData));
    isAuthenticated.set(true);
    console.log('Setting user data in login function:', userData);
    setUser(userData);
  }
}

export function logout() {
  if (browser) {
    localStorage.removeItem('authToken');
    localStorage.removeItem('refreshToken');
    localStorage.removeItem('userData');
    isAuthenticated.set(false);
    setUser(null);
  }
}
/*
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
  */
