<script>
  import Header from '$lib/components/Header.svelte';
  import '../app.css';
  import { ModeWatcher } from "mode-watcher";
  import { isAuthenticated } from '$lib/stores/authStore'
  import { checkAuth, logout } from '$lib/auth';
  import { page } from '$app/stores';
  import { onMount, afterUpdate } from 'svelte';
  import { browser } from '$app/environment';
  import { theme } from '$lib/stores/appStore';
  import { goto } from '$app/navigation';

  let isLoading = true;

  onMount(async () => {
    if (browser) {
      await initializeApp();
    }
  });

  async function initializeApp() {
    await checkAuth();
    
    const storedTheme = localStorage.getItem('theme');
    if (storedTheme) {
      theme.setTheme(storedTheme);
    } else {
      const darkMode = localStorage.getItem('darkMode') === 'true';
      theme.setTheme(darkMode ? 'dark' : 'light');
    }

    isLoading = false;
  }

  $: if (browser && !isLoading) {
    handleRouteChange($page.url.pathname);
  }

  async function handleRouteChange(newPath) {
    const isAuth = await checkAuth();
    console.log('checkAuth', isAuth);

    const publicRoutes = ['/', '/login'];

    if (!isAuth && !publicRoutes.includes(newPath)) {
      console.log('Redirecting to login');
      goto('/login', { replaceState: true });
    } else if (isAuth && newPath === '/login') {
      console.log('Redirecting to home');
      goto('/', { replaceState: true });
    }
  }

  // Subscribe to theme changes
  $: if (browser) {
    theme.subscribe(currentTheme => {
      document.documentElement.classList.toggle('dark', currentTheme === 'dark');
    });
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