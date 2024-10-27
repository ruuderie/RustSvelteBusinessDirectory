import { writable, derived } from 'svelte/store';
import { api } from '$lib/api';
import { browser } from '$app/environment';

export const users = writable([]);
export const loading = writable(true);
export const error = writable(null);

export const pagination = writable({
    currentPage: 1,
    pageSize: 10,
    totalItems: 0
});

export const paginatedUsers = derived(
    [users, pagination],
    ([$users, $pagination]) => {
        const start = ($pagination.currentPage - 1) * $pagination.pageSize;
        const end = start + $pagination.pageSize;
        return $users.slice(start, end);
    }
);

export function nextPage() {
    pagination.update(state => ({
        ...state,
        currentPage: Math.min(state.currentPage + 1, Math.ceil(state.totalItems / state.pageSize))
    }));
}

export function previousPage() {
    pagination.update(state => ({
        ...state,
        currentPage: Math.max(state.currentPage - 1, 1)
    }));
}

export function setUsers(newUsers) {
    users.set(newUsers);
    pagination.update(state => ({ ...state, totalItems: newUsers.length }));
}

function createUserStore() {
    const { subscribe, set } = writable(null);

    return {
        subscribe,
        set: (userData) => {
            if (userData && userData.first_name && userData.last_name) {
                console.log('Setting user data in store:', userData);
                set(userData);
            } else {
                console.warn('User data is incomplete:', userData);
            }
        },
        clear: () => {
            console.log('Clearing user data from store');
            set(null);
        }
    };
}

export const user = createUserStore();

export async function loadUser() {
    if (browser) {
        const storedUser = localStorage.getItem('userData');
        if (storedUser) {
            try {
                const userData = JSON.parse(storedUser);
                console.log('Loading user data from localStorage:', userData);
                if (userData && userData.first_name && userData.last_name) {
                    user.set(userData);
                    return true;
                } else {
                    console.warn('Stored user data is incomplete:', userData);
                    return false;
                }
            } catch (e) {
                console.error('Failed to parse stored user data:', e);
                user.set(null);
                return false;
            }
        }
        return false;
    }
    return false;
}

export function setUser(userData) {
    console.log('Setting user data:', userData);
    user.set(userData);
}

export function clearUser() {
    user.clear();
}
