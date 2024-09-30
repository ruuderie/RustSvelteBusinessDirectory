<script>
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { api } from '$lib/api';  // Update this import

  let profile = null;
  let loading = true;
  let error = null;

  onMount(async () => {
    const id = $page.params.profileId;
    try {
      profile = await api.user.getProfile(id);  // Update this line
      console.log("Profile:", profile);
      loading = false;
    } catch (err) {
      error = err.message;
      loading = false;
    }
  });
</script>

<svelte:head>
  <title>{profile ? profile.username : 'Loading...'} | User Profile</title>
</svelte:head>

<div class="container mx-auto px-4 py-8">
  {#if loading}
    <p class="text-center text-xl">Loading profile details...</p>
  {:else if error}
    <p class="text-center text-xl text-red-500">Error: {error}</p>
  {:else if profile}
    <div class="bg-white shadow-lg rounded-lg overflow-hidden">
      <div class="px-6 py-4">
        <h1 class="text-3xl font-bold mb-4">{profile.username}</h1>
        <p class="text-gray-700 text-xl mb-2"><span class="font-semibold">Email:</span> {profile.email}</p>
        <p class="text-gray-700 text-xl mb-2"><span class="font-semibold">Role:</span> {profile.role}</p>
        <p class="text-gray-700 text-xl mb-2"><span class="font-semibold">Joined:</span> {new Date(profile.created_at).toLocaleDateString()}</p>
        <!-- Add more profile fields as needed -->
      </div>
    </div>
    <div class="mt-8 text-center">
      <a href="/profiles" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
        Back to Profile List
      </a>
    </div>
  {:else}
    <p class="text-center text-xl">Profile not found.</p>
  {/if}
</div>
