<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { isAuthenticated } from '$lib/stores/authStore';
  import { checkAuth } from '$lib/auth';

  onMount(async () => {
    await checkAuth();
    if ($isAuthenticated) {
      goto('/home');
    }
  });
</script>

<div class="container mx-auto px-4 py-8">
    <h1 class="text-3xl font-bold mb-4">Welcome to Oply Command Center</h1>
    <p class="mb-4">Please <a href="/login" class="text-blue-500 hover:underline">log in</a> to access the dashboard.</p>
</div>