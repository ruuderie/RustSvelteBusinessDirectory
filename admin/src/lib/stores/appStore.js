import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

// Import icons
import homeIcon from '$lib/assets/icons/home.svg';
import leadIcon from '$lib/assets/icons/lead.svg';
import customerIcon from '$lib/assets/icons/client.svg';
import dealIcon from '$lib/assets/icons/deal.svg';
import listingIcon from '$lib/assets/icons/announcement.svg';

import userIcon from '$lib/assets/icons/user.svg';
import folderIcon from '$lib/assets/icons/folder.svg';
import templateIcon from '$lib/assets/icons/template.svg';
import reportIcon from '$lib/assets/icons/report.svg';

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

export const selectedTeam = writable('Oply');

const navItems = {
  Oply: [
    { href: "/home", label: "Dashboard", icon: homeIcon },
    { href: "/customers", label: "Clients", icon: customerIcon },
    { href: "/leads", label: "Leads", icon: leadIcon },
    { href: "/deals", label: "Deals", icon: dealIcon },
    { href: "/listings", label: "Listings", icon: listingIcon },
  ],
  Admin: [
    { href: "/home", label: "Dashboard", icon: homeIcon },
    { href: "/listings", label: "Listings", icon: listingIcon },
    { href: "/users", label: "Users", icon: userIcon },
    { href: "/directories", label: "Directories", icon: folderIcon },
    { href: "/templates", label: "Templates", icon: templateIcon },
    { href: "/reports", label: "Reports", icon: reportIcon },
  ]
};

export const currentNavItems = derived(selectedTeam, $selectedTeam => navItems[$selectedTeam]);
