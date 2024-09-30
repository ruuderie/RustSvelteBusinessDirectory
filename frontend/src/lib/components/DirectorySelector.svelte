<script>
    import { onMount } from 'svelte';
    import { selectedDirectoryId, isProduction } from '$lib/stores/directoryStore';
    import { api } from '$lib/api';  // Update this import

    let directories = [];
    let loading = true;
    let error = null;

    onMount(async () => {
        if (!$isProduction) {
            try {
                directories = await api.admin.fetchDirectories();  // Update this line
                if (directories.length > 0 && !$selectedDirectoryId) {
                    $selectedDirectoryId = directories[0].id;
                }
            } catch (e) {
                error = e.message;
            } finally {
                loading = false;
            }
        } else {
            loading = false;
        }
    });

    function handleDirectoryChange(event) {
        $selectedDirectoryId = event.target.value;
    }
</script>

{#if $isProduction}
    <!-- In production, don't show the selector -->
{:else if loading}
    <p>Loading directories...</p>
{:else if error}
    <p class="error">{error}</p>
{:else if directories.length === 0}
    <p>No directories available.</p>
{:else}
    <select on:change={handleDirectoryChange}>
        {#each directories as directory}
            <option value={directory.id} selected={directory.id === $selectedDirectoryId}>{directory.name}</option>
        {/each}
    </select>
{/if}

<style>
    .error {
        color: red;
    }
</style>