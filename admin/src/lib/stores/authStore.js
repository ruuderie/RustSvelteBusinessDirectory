import { writable } from 'svelte/store';

export const isAuthenticated = writable(false);

// Could also have a user store if you're storing user information
// export const user = writable(null);
