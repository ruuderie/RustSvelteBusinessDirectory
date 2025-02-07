<script lang="ts">
    import Ellipsis from "lucide-svelte/icons/ellipsis";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { Button } from "$lib/components/ui/button";
    import { goto } from '$app/navigation';
    
    export let id: string;

    function viewDealDetails(): void {
        goto(`/deals/${id}`);
    }

    function editDeal(): void {
        goto(`/deals/${id}?edit=true`);
    }

    function deactivateDeal(): void {
        goto(`/deals/${id}?deactivate=true`);
    }

    async function copyDealId(): Promise<void> {
        try {
            await navigator.clipboard.writeText(id);
            console.log(`Deal ID ${id} copied to clipboard`);
        } catch (err) {
            console.error('Failed to copy deal ID: ', err);
        }
    }
</script>

<DropdownMenu.Root>
    <DropdownMenu.Trigger asChild let:builder>
        <Button
            variant="ghost"
            builders={[builder]}
            size="icon"
            class="relative h-8 w-8 p-0"
        >
            <span class="sr-only">Open menu</span>
            <Ellipsis class="h-4 w-4" />
        </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content class="w-56 bg-background border border-border rounded-md shadow-md">
        <DropdownMenu.Group>
            <DropdownMenu.Label class="px-2 py-1.5 text-sm font-semibold text-foreground">Actions</DropdownMenu.Label>
            <DropdownMenu.Item on:click={viewDealDetails} class="px-2 py-1.5 text-sm text-foreground hover:bg-accent hover:text-accent-foreground cursor-pointer">
                View deal details
            </DropdownMenu.Item>
            <DropdownMenu.Item on:click={editDeal} class="px-2 py-1.5 text-sm text-foreground hover:bg-accent hover:text-accent-foreground cursor-pointer">
                Edit deal
            </DropdownMenu.Item>
            <DropdownMenu.Item on:click={deactivateDeal} class="px-2 py-1.5 text-sm text-foreground hover:bg-accent hover:text-accent-foreground cursor-pointer">
                Deactivate deal
            </DropdownMenu.Item>
            <DropdownMenu.Item on:click={copyDealId} class="px-2 py-1.5 text-sm text-foreground hover:bg-accent hover:text-accent-foreground cursor-pointer">
                Copy deal ID
            </DropdownMenu.Item>
        </DropdownMenu.Group>
    </DropdownMenu.Content>
</DropdownMenu.Root>