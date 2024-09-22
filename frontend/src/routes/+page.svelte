<script>
  import { onMount } from 'svelte';
  import BusinessCard from '$lib/components/BusinessCard.svelte';
  import SearchBar from '$lib/components/SearchBar.svelte';
  import UserRegistration from '$lib/components/UserRegistration.svelte';
  import UserLogin from '$lib/components/UserLogin.svelte';
  import { fetchListings, searchListings, fetchListingById,registerUser, loginUser } from '$lib/api';
  import DirectorySelector from '$lib/components/DirectorySelector.svelte';
  import { isAuthenticated } from '$lib/auth';
  import { isProduction } from '$lib/stores/directoryStore';

  let businesses = [];
  let searchQuery = '';
  let loading = true;
  let error = null;
  let user = null;
  let showRegistration = false;
  let showLogin = false;
  let registrationError = ''; // Define registrationError here

  onMount(async () => {
    console.log('Fetching businesses...');
    try {
      businesses = await fetchListings();
      console.log('Fetched businesses:', businesses);
    } catch (err) {
      console.error('Error fetching businesses:', err);
      error = err.message;
    } finally {
      loading = false;
    }
  });

  async function handleSearch() {
    console.log('Searching businesses with query:', searchQuery);
    loading = true;
    error = null;
    try {
      businesses = await searchListings(searchQuery);
      console.log('Search results:', businesses);
    } catch (err) {
      console.error('Error searching businesses:', err);
      error = err.message;
    } finally {
      loading = false;
    }
  }

  async function handleRegister(event) {
    try {
      const userData = event.detail;
      const newUser = await registerUser(userData);
      user = newUser;
      showRegistration = false;
      registrationError = '';
    } catch (err) {
      console.error('Error registering user:', err);
      if (err.status === 500) {
        registrationError = 'An internal server error occurred. Please try again later.';
      } else if (err.status === 409) {
        registrationError = 'This email is already registered. Please login instead.';
      } else {
        registrationError = 'An error occurred during registration. Please try again.';
      }
    }
  }

  async function handleLogin(event) {
    try {
      const credentials = event.detail;
      const token = await loginUser(credentials);
      // Store the token in localStorage or a secure cookie
      localStorage.setItem('token', token);
      user = { email: credentials.email }; // fetch user details here
      showLogin = false;
    } catch (err) {
      console.error('Error logging in:', err);
      error = err.message;
    }
  }

  function handleLogout() {
    localStorage.removeItem('token');
    user = null;
  }
</script>

<svelte:head>
  <title>Business Directory</title>
</svelte:head>

{#if !$isProduction}
  <DirectorySelector />
{/if}

{#if $isAuthenticated}
  <DirectorySelector />
  <!-- Rest of your authenticated content -->
{:else}
  <UserLogin on:login={handleLogin} />
{/if}

<div class="container mx-auto px-4 py-8">
  <h1 class="text-3xl font-bold text-center mb-8">Business Directory</h1>
  
  <DirectorySelector />

  {#if user}
    <div class="mb-4">
      <p>Welcome, {user.email}!</p>
      <button on:click={handleLogout}>Logout</button>
    </div>
  {:else}
    <div class="mb-4">
      <button on:click={() => showRegistration = true}>Register</button>
      <button on:click={() => showLogin = true}>Login</button>
    </div>
  {/if}

  {#if showRegistration}
    <UserRegistration on:register={handleRegister} errorMessage={registrationError} />
  {/if}

  {#if showLogin}
    <UserLogin on:login={handleLogin} />
  {/if}

  <div class="mb-8">
    <SearchBar bind:value={searchQuery} on:search={handleSearch} />
  </div>

  {#if loading}
    <p class="text-center">Loading businesses...</p>
  {:else if error}
    <p class="text-center text-red-500">Error: {error}</p>
  {:else if businesses.length === 0}
    <p class="text-center">No businesses found.</p>
  {:else}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
      {#each businesses as business (business.id)}
        <BusinessCard {business} />
      {/each}
    </div>
  {/if}
</div>
