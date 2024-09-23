<script lang="ts">
  import { isAuthenticated, logout } from '$lib/auth';
  import { Button } from "$lib/components/ui/button";
  import { Avatar, AvatarFallback, AvatarImage } from "$lib/components/ui/avatar";
  import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger } from '$lib/components/ui/dropdown-menu';
  import { LogOut, UserPlus, Settings, HelpCircle, Command } from 'lucide-svelte';
</script>

<header class="bg-background border-b border-border">
  <nav class="container mx-auto px-4 py-3 flex justify-between items-center">
    <a href="/" class="text-xl font-semibold text-foreground flex items-center">
      <Command class="w-8 h-8 mr-2 text-primary" />
      <span>Oply Command Center</span>
    </a>
    <div class="flex items-center space-x-4">
      {#if $isAuthenticated}
        <Button variant="ghost" class="text-muted-foreground hover:text-foreground">
          <HelpCircle class="w-5 h-5 mr-2" />
          Help
        </Button>
        <DropdownMenu>
          <DropdownMenuTrigger>
            <Avatar class="w-8 h-8 transition duration-300 ease-in-out transform hover:scale-105">
              <!-- <AvatarImage src="/path-to-user-image.jpg" alt="User" /> -->
              <AvatarFallback class="bg-muted text-muted-foreground">CN</AvatarFallback>
            </Avatar>
          </DropdownMenuTrigger>
          <DropdownMenuContent class="w-56">
            <DropdownMenuLabel class="font-normal">
              <div class="flex flex-col space-y-1">
                <p class="text-sm font-medium leading-none">Charlie North</p>
                <p class="text-xs leading-none text-muted-foreground">charlie@example.com</p>
              </div>
            </DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuItem>
              <Settings class="mr-2 h-4 w-4" />
              <span>Settings</span>
            </DropdownMenuItem>
            <DropdownMenuItem on:click={logout}>
              <LogOut class="mr-2 h-4 w-4" />
              <span>Log out</span>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      {:else}
        <Button variant="ghost" href="/login" class="text-muted-foreground hover:text-foreground">
          Login
        </Button>
        <Button variant="default" href="/register" class="bg-primary text-primary-foreground hover:bg-primary/90">
          <UserPlus class="mr-2 h-4 w-4" />
          Register
        </Button>
      {/if}
    </div>
  </nav>
</header>