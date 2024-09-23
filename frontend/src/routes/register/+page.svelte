<script>
    import UserRegistration from '$lib/components/UserRegistration.svelte';
    import { registerUser } from '$lib/api';
  
    let errorMessage = '';
  
    async function handleRegister(event) {
      const { username, email, password } = event.detail;
      try {
        await registerUser({ username, email, password });
        window.location.href = '/login';
      } catch (err) {
        errorMessage = 'Registration failed. Please try again.';
      }
    }
  </script>
  
  <UserRegistration on:register={handleRegister} {errorMessage} />
  
  {#if errorMessage}
    <p class="text-red-500 text-center mt-4">{errorMessage}</p>
  {/if}