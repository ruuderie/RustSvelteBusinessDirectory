<script>
  import { createEventDispatcher } from 'svelte';
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";

  const dispatch = createEventDispatcher();

  let email = '';
  let password = '';

  async function handleSubmit() {
    dispatch('login', { email, password });
  }
</script>

<form on:submit|preventDefault={handleSubmit} class="space-y-4 bg-white p-6 rounded-lg shadow-md">
  <div>
    <Label for="email">Email</Label>
    <Input id="email" type="email" bind:value={email} required />
  </div>
  <div>
    <Label for="password">Password</Label>
    <Input id="password" type="password" bind:value={password} required />
  </div>
  <Button type="submit" class="w-full bg-blue-600 text-white py-2 rounded">Login</Button>
</form>

<style>
  form {
    max-width: 400px;
    margin: 0 auto;
  }
</style>