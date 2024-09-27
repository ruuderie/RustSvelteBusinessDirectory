import { writable, derived } from 'svelte/store';

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