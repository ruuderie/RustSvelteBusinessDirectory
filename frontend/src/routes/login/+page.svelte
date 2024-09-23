<script>
    import UserLogin from '$lib/components/UserLogin.svelte';
    import { loginUser } from '$lib/api';
    import { login } from '$lib/auth';
  
    let errorMessage = '';
  
    async function handleLogin(event) {
      const { email, password } = event.detail;
      try {
        const token = await loginUser({ email, password });
        login(token);
        window.location.href = '/';
      } catch (err) {
        errorMessage = 'Login failed. Please check your credentials and try again.';
      }
    }
  </script>
  
  <UserLogin on:login={handleLogin} />
  
  {#if errorMessage}
    <p class="text-red-500 text-center mt-4">{errorMessage}</p>
  {/if}