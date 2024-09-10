<script>
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { fetchBusinessById } from '$lib/api';

  let business = null;

  onMount(async () => {
    const id = $page.params.id;
    business = await fetchBusinessById(id);
  });
</script>

<svelte:head>
  <title>{business ? business.name : 'Loading...'} | Business Directory</title>
</svelte:head>

{#if business}
  <h1>{business.name}</h1>
  <p>Category: {business.category}</p>
  <p>Address: {business.address}</p>
  <p>Phone: {business.phone}</p>
  <p>Website: <a href={business.website} target="_blank" rel="noopener noreferrer">{business.website}</a></p>
{:else}
  <p>Loading...</p>
{/if}

<style>
  h1 {
    margin-bottom: 20px;
  }

  p {
    margin-bottom: 10px;
  }
</style>
