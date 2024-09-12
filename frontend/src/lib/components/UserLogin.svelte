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

<form on:submit|preventDefault={handleSubmit} class="space-y-4">
  <div>
    <Label for="email">Email</Label>
    <Input id="email" type="email" bind:value={email} required />
  </div>
  <div>
    <Label for="password">Password</Label>
    <Input id="password" type="password" bind:value={password} required />
  </div>
  <Button type="submit">Login</Button>
</form>
