<script>
    import { createEventDispatcher } from 'svelte';
    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { cn } from "$lib/utils";
    import { Lock, Loader2, UserPlus } from 'lucide-svelte';
    import { loginUserMock } from '$lib/api';
    import { isAuthenticated } from '$lib/auth';  // Import the isAuthenticated store
  
    const dispatch = createEventDispatcher();
  
    let className = undefined;
    export { className as class };
    export let mode = 'login'; // 'login' or 'register'
  
    let isLoading = false;
    let username = '';
    let email = '';
    let password = '';
    let errorMessage = '';
  
    async function onSubmit() {
      isLoading = true;
      errorMessage = '';
      
      try {
        if (mode === 'login') {
          const token = await loginUserMock({ email, password });
          localStorage.setItem('authToken', token);
          isAuthenticated.set(true);  // Update the authentication state
          dispatch('login', { email, password });
        } else {
          dispatch('register', { username, email, password });
          console.log('Registration attempted:', { username, email, password });
        }
      } catch (error) {
        errorMessage = error.message;
      } finally {
        isLoading = false;
      }
    }
  </script>
  
  <div class={cn("grid gap-6", className)} {...$$restProps}>
    <form on:submit|preventDefault={onSubmit}>
      <div class="grid gap-2">
        {#if mode === 'register'}
          <div class="grid gap-1">
            <Label for="username">Username</Label>
            <Input
              id="username"
              bind:value={username}
              placeholder="johndoe"
              autocomplete="username"
              disabled={isLoading}
            />
          </div>
        {/if}
        <div class="grid gap-1">
          <Label for="email">Email</Label>
          <Input
            id="email"
            bind:value={email}
            placeholder="admin@oply.com"
            type="email"
            autocapitalize="none"
            autocomplete="email"
            autocorrect="off"
            disabled={isLoading}
          />
        </div>
        <div class="grid gap-1">
          <Label for="password">Password</Label>
          <Input
            id="password"
            bind:value={password}
            placeholder="••••••••"
            type="password"
            autocapitalize="none"
            autocomplete={mode === 'login' ? "current-password" : "new-password"}
            disabled={isLoading}
          />
        </div>
        {#if errorMessage}
          <p class="text-red-500 text-sm">{errorMessage}</p>
        {/if}
        <Button type="submit" disabled={isLoading}>
          {#if isLoading}
            <Loader2 class="mr-2 h-4 w-4 animate-spin" />
          {:else if mode === 'login'}
            <Lock class="mr-2 h-4 w-4" />
          {:else}
            <UserPlus class="mr-2 h-4 w-4" />
          {/if}
          {mode === 'login' ? 'Login to Command Center' : 'Register'}
        </Button>
      </div>
    </form>
  </div>