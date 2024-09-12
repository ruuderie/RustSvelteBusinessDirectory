<script>
  import { createEventDispatcher } from 'svelte';
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";

  const dispatch = createEventDispatcher();

  let username = '';
  let email = '';
  let password = '';
  export let errorMessage = ''; // Add this line to accept the errorMessage prop

  function handleSubmit() {
    dispatch('register', { username, email, password });
  }
</script>

<form on:submit|preventDefault={handleSubmit} class="space-y-4">
  <div>
    <Label for="username">Username</Label>
    <Input id="username" bind:value={username} required />
  </div>
  <div>
    <Label for="email">Email</Label>
    <Input id="email" type="email" bind:value={email} required />
  </div>
  <div>
    <Label for="password">Password</Label>
    <Input id="password" type="password" bind:value={password} required />
  </div>
  
  {#if errorMessage}
    <p class="text-red-500">{errorMessage}</p>
  {/if}
  
  <Button type="submit">Register</Button>
</form>
