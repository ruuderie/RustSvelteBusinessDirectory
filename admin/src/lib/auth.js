import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import { isAuthenticated } from './stores/authStore';


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
  }
}

export function checkAuth() {
  if (browser) {
    const token = localStorage.getItem('authToken');
    isAuthenticated.set(!!token);
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
