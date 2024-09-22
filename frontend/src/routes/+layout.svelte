<script>
  import { selectedDirectoryId, isProduction } from '$lib/stores/directoryStore';
  import DirectorySelector from '$lib/components/DirectorySelector.svelte';

  $: directorySelected = $selectedDirectoryId !== null;
</script>

{#if !directorySelected && !$isProduction}
  <p>Please select a directory to continue:</p>
  <DirectorySelector />
{:else if !directorySelected && $isProduction}
  <p>Error: No directory configured. Please contact the administrator.</p>
{:else}
  <slot />
{/if}

<style>
  main {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
  }
</style>
