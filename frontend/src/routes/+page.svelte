<script>
  import { onMount } from 'svelte';
  import BusinessCard from '$lib/components/BusinessCard.svelte';
  import SearchBar from '$lib/components/SearchBar.svelte';
  import { fetchBusinesses, searchBusinesses } from '$lib/api';

  let businesses = [];
  let searchQuery = '';
  let loading = true;
  let error = null;

  onMount(async () => {
    console.log('Fetching businesses...');
    try {
      businesses = await fetchBusinesses();
      console.log('Fetched businesses:', businesses);
    } catch (err) {
      console.error('Error fetching businesses:', err);
      error = err.message;
    } finally {
      loading = false;
    }
  });

  async function handleSearch() {
    console.log('Searching businesses with query:', searchQuery);
    loading = true;
    error = null;
    try {
      businesses = await searchBusinesses(searchQuery);
      console.log('Search results:', businesses);
    } catch (err) {
      console.error('Error searching businesses:', err);
      error = err.message;
    } finally {
      loading = false;
    }
  }
</script>

<svelte:head>
  <title>Business Directory</title>
</svelte:head>

<h1>Business Directory</h1>

<SearchBar bind:value={searchQuery} on:search={handleSearch} />

{#if loading}
  <p>Loading businesses...</p>
{:else if error}
  <p>Error: {error}</p>
{:else if businesses.length === 0}
  <p>No businesses found.</p>
{:else}
  <div class="business-list">
    {#each businesses as business (business.id)}
      <BusinessCard {business} />
    {/each}
  </div>
{/if}

<style>
  h1 {
    text-align: center;
    margin-bottom: 20px;
  }

  .business-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 20px;
  }
</style>
