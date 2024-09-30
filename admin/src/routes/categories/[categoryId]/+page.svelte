<script>
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { api } from '$lib/api';

  let category = null;
  let loading = true;
  let error = null;

  onMount(async () => {
    const id = $page.params.id;
    try {
      category = await api.admin?.fetchCategoryById(id);
      console.log("Category:", category);
      loading = false;
    } catch (err) {
      error = err.message;
      loading = false;
    }
  });
</script>

<svelte:head>
  <title>{category ? category.name : 'Loading...'} | Category</title>
</svelte:head>

<div class="container mx-auto px-4 py-8">
  {#if loading}
    <p class="text-center text-xl">Loading category details...</p>
  {:else if error}
    <p class="text-center text-xl text-red-500">Error: {error}</p>
  {:else if category}
    <div class="bg-white shadow-lg rounded-lg overflow-hidden">
      <div class="px-6 py-4">
        <h1 class="text-3xl font-bold mb-4">{category.title}</h1>
        <p class="text-gray-700 text-xl mb-2"><span class="font-semibold">Category:</span> {category.category_id}</p>
        <p class="text-gray-700 text-xl mb-2"><span class="font-semibold">Address:</span> {category.city}, {category.state} {category.neighborhood}</p>
        <p class="text-gray-700 text-xl mb-2"><span class="font-semibold">Phone:</span> {category.phone}</p>
       
      </div>
    </div>
    <div class="mt-8 text-center">
      <a href="/" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
        Back to Category List
      </a>
    </div>
  {:else}
    <p class="text-center text-xl">Category not found.</p>
  {/if}
</div>
