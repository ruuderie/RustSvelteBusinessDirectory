<script>
  import { onMount } from 'svelte';
  import { api } from '$lib/api';  // Update this import
  import ProfileCard from '$lib/components/ProfileCard.svelte';
  import SearchBar from '$lib/components/SearchBar.svelte';
  import { writable } from 'svelte/store';

  let profiles = [];
  let filteredProfiles = [];
  let loading = true;
  let error = null;

  const searchQuery = writable('');

  onMount(async () => {
    try {
      profiles = await api.admin.fetchUsers();  // Update this line
      filteredProfiles = profiles;
      loading = false;
    } catch (err) {
      error = err.message;
      loading = false;
    }
  });

  $: {
    if (profiles.length > 0) {
      filteredProfiles = profiles.filter(profile =>
        profile.username.toLowerCase().includes($searchQuery.toLowerCase()) ||
        profile.email.toLowerCase().includes($searchQuery.toLowerCase())
      );
    }
  }

  function handleSearch(event) {
    searchQuery.set(event.detail);
  }
</script>

<svelte:head>
  <title>User Profiles | Admin Dashboard</title>
</svelte:head>

<div class="container mx-auto px-4 py-8">
  <h1 class="text-3xl font-bold mb-8">User Profiles</h1>

  <SearchBar on:search={handleSearch} />

  {#if loading}
    <p class="text-center text-xl mt-8">Loading profiles...</p>
  {:else if error}
    <p class="text-center text-xl text-red-500 mt-8">Error: {error}</p>
  {:else if filteredProfiles.length > 0}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-8">
      {#each filteredProfiles as profile (profile.id)}
        <ProfileCard {profile} />
      {/each}
    </div>
  {:else}
    <p class="text-center text-xl mt-8">No profiles found.</p>
  {/if}
</div>
