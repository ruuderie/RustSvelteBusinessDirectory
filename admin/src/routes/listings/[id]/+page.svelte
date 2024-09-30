<script>
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { api } from '$lib/api';  // Update this import

  let listing = null;
  let loading = true;
  let error = null;

  onMount(async () => {
    const id = $page.params.id;
    try {
      listing = await api.listing.fetchListingById(id);  // Update this line
      console.log("Listing:", listing);
      loading = false;
    } catch (err) {
      error = err.message;
      loading = false;
    }
  });
</script>

<svelte:head>
  <title>{listing ? listing.name : 'Loading...'} | Listing Details</title>
</svelte:head>

<div class="container mx-auto px-4 py-8">
  {#if loading}
    <p class="text-center text-xl">Loading listing details...</p>
  {:else if error}
    <p class="text-center text-xl text-red-500">Error: {error}</p>
  {:else if listing}
    <div class="bg-white shadow-lg rounded-lg overflow-hidden">
      <div class="px-6 py-4">
        <h1 class="text-3xl font-bold mb-4">{listing.title}</h1>
        <p class="text-gray-700 text-xl mb-2"><span class="font-semibold">Category:</span> {listing.category_id}</p>
        <p class="text-gray-700 text-xl mb-2"><span class="font-semibold">Address:</span> {listing.city}, {listing.state} {listing.neighborhood}</p>
        <p class="text-gray-700 text-xl mb-2"><span class="font-semibold">Phone:</span> {listing.phone}</p>
        <p class="text-gray-700 text-xl mb-2">
          <span class="font-semibold">Website:</span> 
          <a href={listing.website} target="_blank" rel="noopener noreferrer" class="text-blue-500 hover:underline">
            {listing.website}
          </a>
        </p>
      </div>
    </div>
    <div class="mt-8 text-center">
      <a href="/" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
        Back to Listing List
      </a>
    </div>
  {:else}
    <p class="text-center text-xl">Listing not found.</p>
  {/if}
</div>
