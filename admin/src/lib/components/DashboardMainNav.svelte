<script>
  import { page } from '$app/stores';
  import { cn } from "$lib/utils";
  import { currentNavItems } from '$lib/stores/appStore';

  let className = '';
  export { className as class };

  $: currentPath = $page.url.pathname;
</script>

<nav class={cn("flex items-center space-x-4 lg:space-x-6", className)}>
  {#each $currentNavItems as item}
    <a
      href={item.href}
      class={cn(
        "text-sm font-medium transition-colors hover:text-primary flex items-center",
        currentPath === item.href
          ? "text-primary"
          : "text-muted-foreground"
      )}
    >
      <img src={item.icon} alt="" class="w-4 h-4 mr-2" />
      {item.label}
    </a>
  {/each}
</nav>