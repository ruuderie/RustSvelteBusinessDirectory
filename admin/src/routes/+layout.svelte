<script>
  import Header from '$lib/components/Header.svelte';
  import '../app.css';
  import { ModeWatcher } from "mode-watcher";
  import {isAuthenticated } from '$lib/stores/authStore'
  import {  checkAuth } from '$lib/auth';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { browser } from '$app/environment';

  onMount(() => {
    const darkMode = localStorage.getItem('darkMode') === 'true';
    document.documentElement.classList.toggle('dark', darkMode);
  });

  onMount(() => {
    if (browser) {
      checkAuth();
    }
  });

  $: if (browser && $page) {
    checkAuth();
  }
</script>

<div class="min-h-screen flex flex-col bg-background text-foreground">
  <Header />

  <main class="flex-grow container mx-auto px-4 py-8">
    {#if $page.url.pathname === '/login' || $isAuthenticated}
      <slot />
    {:else}
      <div class="flex items-center justify-center min-h-screen">
        <a href="/login" class="text-primary hover:underline">Please log in to access the dashboard</a>
      </div>
    {/if}
  </main>

  <footer class="bg-background border-t">
    <div class="container mx-auto px-4 py-4 text-center text-sm text-muted-foreground">
      © 2023 Oply Command Center. All rights reserved.
    </div>
  </footer>
</div>