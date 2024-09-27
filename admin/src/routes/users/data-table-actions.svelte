<script lang="ts">
    import Ellipsis from "lucide-svelte/icons/ellipsis";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { Button } from "$lib/components/ui/button";
    import { goto } from '$app/navigation';
    
    export let id: string;

    function viewUserDetails() {
        goto(`/users/${id}`);
    }

    function editUser() {
        // TODO: Implement edit user functionality
        console.log(`Edit user with ID: ${id}`);
    }

    function deactivateUser() {
        // TODO: Implement deactivate user functionality
        console.log(`Deactivate user with ID: ${id}`);
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
    <DropdownMenu.Content>
        <DropdownMenu.Group>
            <DropdownMenu.Label>Actions</DropdownMenu.Label>
            <DropdownMenu.Item on:click={viewUserDetails}>
                View user details
            </DropdownMenu.Item>
            <DropdownMenu.Item on:click={editUser}>
                Edit user
            </DropdownMenu.Item>
            <DropdownMenu.Item on:click={deactivateUser}>
                Deactivate user
            </DropdownMenu.Item>
            <DropdownMenu.Item on:click={() => navigator.clipboard.writeText(id)}>
                Copy user ID
            </DropdownMenu.Item>
        </DropdownMenu.Group>
    </DropdownMenu.Content>
</DropdownMenu.Root>