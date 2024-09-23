<script>
  import { isAuthenticated, logout } from '$lib/auth';
</script>

<header class="bg-blue-600 text-white p-4">
  <nav class="container mx-auto flex justify-between items-center">
    <a href="/" class="text-2xl font-bold">Builder Connect</a>
    <div>
      {#if $isAuthenticated}
        <button on:click={logout} class="ml-4">Logout</button>
      {:else}
        <a href="/login" class="ml-4">Login</a>
        <a href="/register" class="ml-4">Register</a>
      {/if}
    </div>
  </nav>
</header>

<style>
  a {
    text-decoration: none;
    color: white;
    font-size: 18px;
    font-weight: bold;
  }

  button {
    background: none;
    border: none;
    color: white;
    font-size: 18px;
    font-weight: bold;
    cursor: pointer;
  }
</style>