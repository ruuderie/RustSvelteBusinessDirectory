<script>
    import { page } from '$app/stores';
    import { cn } from "$lib/utils";
    import { browser } from '$app/environment';
  
    let className = '';
    export { className as class };
  
    const navItems = [
      { href: "/", label: "Dashboard" },
      { href: "/users", label: "Users" },
      { href: "/directories", label: "Directories" },
      { href: "/listings", label: "Listings" },
      { href: "/ad-purchases", label: "Ad Purchases" },
      { href: "/categories", label: "Categories" },
      { href: "/profiles", label: "Profiles" },
      { href: "/templates", label: "Templates" },
      { href: "/statistics", label: "Statistics" },
      { href: "/reports", label: "Reports" },
    ];
  
    $: currentPath = browser ? $page.url.pathname : '/';
  </script>
  
  <nav class={cn("flex items-center space-x-4 lg:space-x-6", className)}>
    {#each navItems as item}
      <a
        href={item.href}
        class={cn(
          "text-sm font-medium transition-colors hover:text-primary",
          currentPath === item.href
            ? "text-primary"
            : "text-muted-foreground"
        )}
      >
        {item.label}
      </a>
    {/each}
  </nav>