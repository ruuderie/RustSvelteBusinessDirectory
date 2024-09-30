import { writable } from 'svelte/store';
import { browser } from '$app/environment';

function createThemeStore() {
    const defaultTheme = 'light';
    const initialTheme = browser ? window.localStorage.getItem('theme') || defaultTheme : defaultTheme;

    const { subscribe, set, update } = writable(initialTheme);

    return {
        subscribe,
        toggleTheme: () => update(theme => {
            const newTheme = theme === 'light' ? 'dark' : 'light';
            if (browser) {
                window.localStorage.setItem('theme', newTheme);
                document.documentElement.classList.toggle('dark', newTheme === 'dark');
            }
            return newTheme;
        }),
        setTheme: (newTheme) => {
            set(newTheme);
            if (browser) {
                window.localStorage.setItem('theme', newTheme);
                document.documentElement.classList.toggle('dark', newTheme === 'dark');
            }
        }
    };
}

export const theme = createThemeStore();
