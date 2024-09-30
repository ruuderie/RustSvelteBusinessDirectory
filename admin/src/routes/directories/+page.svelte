<script>
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { api } from '$lib/api';

  let directory = null;
  let loading = true;
  let error = null;

  onMount(async () => {
    const id = $page.params.id;
    try {
      directory = await api.admin?.fetchDirectoryById(id);
      console.log("Directory:", directory);
      loading = false;
    } catch (err) {
      error = err.message;
      loading = false;
    }
  });
</script>

<svelte:head>
  <title>{directory ? directory.name : 'Loading...'} | Listing Directory</title>
</svelte:head>

<div class="container mx-auto px-4 py-8">
  {#if loading}
    <p class="text-center text-xl">Loading listing details...</p>
  {:else if error}
    <p class="text-center text-xl text-red-500">Error: {error}</p>
  {:else if directory}
    <div class="bg-white shadow-lg rounded-lg overflow-hidden">
      <div class="px-6 py-4">
        <h1 class="text-3xl font-bold mb-4">{directory.title}</h1>
        <p class="text-gray-700 text-xl mb-2"><span class="font-semibold">Category:</span> {directory.category_id}</p>
        <p class="text-gray-700 text-xl mb-2"><span class="font-semibold">Address:</span> {directory.city}, {directory.state} {directory.neighborhood}</p>
        <p class="text-gray-700 text-xl mb-2"><span class="font-semibold">Phone:</span> {directory.phone}</p>
        <p class="text-gray-700 text-xl mb-2">
          <span class="font-semibold">Website:</span> 
          <a href={directory.website} target="_blank" rel="noopener noreferrer" class="text-blue-500 hover:underline">
            {directory.website}
          </a>
        </p>
      </div>
    </div>
    <div class="mt-8 text-center">
      <a href="/" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
        Back to Directory List
      </a>
    </div>
  {:else}
    <p class="text-center text-xl">Directory not found.</p>
  {/if}
</div>
