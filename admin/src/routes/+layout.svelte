<script lang="ts">
  import Header from '$lib/components/Header.svelte';
</script>

<div class="min-h-screen flex flex-col bg-background text-foreground">
  <Header />

  <main class="flex-grow container mx-auto px-4 py-8">
    <slot />
  </main>

  <footer class="bg-background border-t">
    <div class="container mx-auto px-4 py-4 text-center text-sm text-muted-foreground">
      © 2023 Oply Command Center. All rights reserved.
    </div>
  </footer>
</div>