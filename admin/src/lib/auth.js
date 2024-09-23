import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export const isAuthenticated = writable(false);

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
