<script>
  import Header from '$lib/components/Header.svelte';
</script>

<Header />

<main class="container mx-auto p-4">
  <slot />
</main>

<style>
  main {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
  }
</style>
