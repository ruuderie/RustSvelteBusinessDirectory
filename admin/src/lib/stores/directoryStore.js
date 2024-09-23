import { writable, readable, derived } from 'svelte/store';

const initialDirectoryId = import.meta.env.VITE_DIRECTORY_ID || null;
export const selectedDirectoryId = writable(initialDirectoryId);
export const isProduction = readable(import.meta.env.PROD);

// This derived store will always have a value in production
export const effectiveDirectoryId = derived(
    [selectedDirectoryId, isProduction],
    ([$selectedDirectoryId, $isProduction]) => {
        if ($isProduction) {
            return initialDirectoryId;
        }
        return $selectedDirectoryId;
    }
);