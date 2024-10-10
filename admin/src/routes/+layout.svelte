<script>
  import '../app.css';
  import { ModeWatcher } from "mode-watcher";
  import { theme } from '$lib/stores/appStore';
  import { browser } from '$app/environment';
  import { isAuthenticated } from '$lib/stores/authStore';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { checkAuth } from '$lib/auth';
  import { onMount } from 'svelte';
  import Header from '$lib/components/Header.svelte';

  let isLoading = true;

  onMount(async () => {
    if (browser) {
      console.log('Initializing app');
      await initializeApp();
    }
  });

  async function initializeApp() {
    const isAuth = await checkAuth();
    isAuthenticated.set(isAuth);
    isLoading = false;

    // Redirect based on authentication status
    if (isAuth && $page.url.pathname === '/') {
      goto('/home');
    } else if (!isAuth && $page.url.pathname !== '/login' && $page.url.pathname !== '/register' || $page.url.pathname == '/(authenticated)') {
      goto('/');
    }

    
  }

  // Subscribe to theme changes
  $: if (browser) {
    theme.subscribe(currentTheme => {
      document.documentElement.classList.toggle('dark', currentTheme === 'dark');
    });
  }

  // Watch for route changes
  $: if (browser && !isLoading) {
    handleRouteChange($page.url.pathname);
  }

  async function handleRouteChange(path) {
    const isAuth = await checkAuth();
    if (!isAuth && path !== '/login' && path !== '/register' && path !== '/') {
      goto('/');
    } else if (isAuth && path === '/') {
      goto('/home');
    }
  }
</script>

{#if isLoading}
  <div class="flex items-center justify-center min-h-screen">
    Loading...
  </div>
{:else}
  <div class="min-h-screen flex flex-col bg-background text-foreground">
    <Header />

    <main class="flex-grow container mx-auto px-4 py-8">
      <slot />
    </main>

    <footer class="bg-background border-t">
      <div class="container mx-auto px-4 py-4 text-center text-sm text-muted-foreground">
        © 2023 Oply Command Center. All rights reserved.
      </div>
    </footer>
  </div>
{/if}