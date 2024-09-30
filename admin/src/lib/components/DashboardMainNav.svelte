<script>
    import { page } from '$app/stores';
    import { cn } from "$lib/utils";

    // Import icons
    import homeIcon from '$lib/assets/icons/home.svg';
    import userIcon from '$lib/assets/icons/user.svg';
    import folderIcon from '$lib/assets/icons/folder.svg';
    import announcementIcon from '$lib/assets/icons/announcement.svg';
    import adIcon from '$lib/assets/icons/campaign.svg';
    import profileIcon from '$lib/assets/icons/avatar.svg';
    import templateIcon from '$lib/assets/icons/template.svg';
    import reportIcon from '$lib/assets/icons/report.svg';

    let className = '';
    export { className as class };

    const navItems = [
      { href: "/", label: "Dashboard", icon: homeIcon },
      { href: "/users", label: "Users", icon: userIcon },
      { href: "/directories", label: "Directories", icon: folderIcon },
      { href: "/listings", label: "Listings", icon: announcementIcon },
      { href: "/ad-purchases", label: "Ad Purchases", icon: adIcon },
      { href: "/profiles", label: "Profiles", icon: profileIcon },
      { href: "/templates", label: "Templates", icon: templateIcon },
      { href: "/reports", label: "Reports", icon: reportIcon },
    ];

    $: currentPath = $page.url.pathname;
</script>

<nav class={cn("flex items-center space-x-4 lg:space-x-6", className)}>
    {#each navItems as item}
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