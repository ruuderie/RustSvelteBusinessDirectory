<script>
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { api } from '$lib/api';  // Update this import

  let template = null;
  let loading = true;
  let error = null;

  onMount(async () => {
    const id = $page.params.id;
    try {
      template = await api.template.fetchTemplateById(id);  // Update this line
      console.log("Template:", template);
      loading = false;
    } catch (err) {
      error = err.message;
      loading = false;
    }
  });
</script>

<svelte:head>
  <title>{template ? template.name : 'Loading...'} | Template Directory</title>
</svelte:head>

<div class="container mx-auto px-4 py-8">
  {#if loading}
    <p class="text-center text-xl">Loading template details...</p>
  {:else if error}
    <p class="text-center text-xl text-red-500">Error: {error}</p>
  {:else if template}
    <div class="bg-white shadow-lg rounded-lg overflow-hidden">
      <div class="px-6 py-4">
        <h1 class="text-3xl font-bold mb-4">{template.title}</h1>
        <p class="text-gray-700 text-xl mb-2"><span class="font-semibold">Category:</span> {template.category_id}</p>
        <p class="text-gray-700 text-xl mb-2"><span class="font-semibold">Description:</span> {template.description}</p>
        <p class="text-gray-700 text-xl mb-2"><span class="font-semibold">Created By:</span> {template.created_by}</p>
        <p class="text-gray-700 text-xl mb-2">
          <span class="font-semibold">Preview:</span> 
          <a href={template.preview_url} target="_blank" rel="noopener noreferrer" class="text-blue-500 hover:underline">
            View Preview
          </a>
        </p>
      </div>
    </div>
    <div class="mt-8 text-center">
      <a href="/templates" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
        Back to Template List
      </a>
    </div>
  {:else}
    <p class="text-center text-xl">Template not found.</p>
  {/if}
</div>
