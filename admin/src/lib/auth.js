import { writable } from 'svelte/store';

export const isAuthenticated = writable(false);

export function login(token) {
  localStorage.setItem('authToken', token);
  isAuthenticated.set(true);
}

export function logout() {
  localStorage.removeItem('authToken');
  isAuthenticated.set(false);
}

export function checkAuth() {
  const token = localStorage.getItem('authToken');
  isAuthenticated.set(!!token);
}
