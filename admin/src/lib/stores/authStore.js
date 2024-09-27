import { writable } from 'svelte/store';

export const isAuthenticated = writable(false);
// Get the env variable for the environment
export const env = import.meta.env.VITE_ENV || 'development';

// Could also have a user store if you're storing user information
// export const user = writable(null);
