<script>
    import { onMount } from 'svelte';
    import { readable } from 'svelte/store';
    import { fetchUsers } from '$lib/api';
    import DataTableActions from "./data-table-actions.svelte";
    import DataTableCheckbox from "./data-table-checkbox.svelte";
    import * as Table from "$lib/components/ui/table";
    import { Button } from "$lib/components/ui/button";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { Input } from "$lib/components/ui/input";
    import { Render, Subscribe, createRender, createTable } from "svelte-headless-table";
    import {
        addHiddenColumns,
        addPagination,
        addSelectedRows,
        addSortBy,
        addTableFilter,
    } from "svelte-headless-table/plugins";
    import { cn } from "$lib/utils.js";
    
    // Import the icons
    import { ChevronDown, ChevronUp } from 'lucide-svelte';

    let users = [];
    let loading = true;
    let error = null;

    onMount(async () => {
        try {
            users = await fetchUsers();
            loading = false;
            initializeTable();
        } catch (err) {
            error = err.message;
            loading = false;
        }
    });

    let table;
    let tableProps;

    function initializeTable() {
        table = createTable(readable(users), {
            sort: addSortBy({ disableMultiSort: true }),
            page: addPagination(),
            filter: addTableFilter({
                fn: ({ filterValue, value }) => value.toLowerCase().includes(filterValue.toLowerCase()),
            }),
            select: addSelectedRows(),
            hide: addHiddenColumns(),
        });

        const columns = table.createColumns([
            table.column({
                header: (_, { pluginStates }) => {
                    const { allPageRowsSelected } = pluginStates.select;
                    return createRender(DataTableCheckbox, {
                        checked: allPageRowsSelected,
                    });
                },
                accessor: "id",
                cell: ({ row }, { pluginStates }) => {
                    const { getRowState } = pluginStates.select;
                    const { isSelected } = getRowState(row);

                    return createRender(DataTableCheckbox, {
                        checked: isSelected,
                    });
                },
                plugins: {
                    sort: { disable: true },
                    filter: { exclude: true },
                },
            }),
            table.column({
                header: "Username",
                accessor: "username",
            }),
            table.column({
                header: "Email",
                accessor: "email",
                cell: ({ value }) => value.toLowerCase(),
                plugins: {
                    filter: {
                        getFilterValue(value) {
                            return value.toLowerCase();
                        },
                    },
                },
            }),
            table.column({
                header: "Admin",
                accessor: "is_admin",
                cell: ({ value }) => value ? 'Yes' : 'No',
            }),
            table.column({
                header: "Active",
                accessor: "is_active",
                cell: ({ value }) => value ? 'Yes' : 'No',
            }),
            table.column({
                header: "Created At",
                accessor: "created_at",
                cell: ({ value }) => new Date(value).toLocaleString(),
            }),
            table.column({
                header: "",
                accessor: ({ id }) => id,
                cell: (item) => createRender(DataTableActions, { id: item.value }),
                plugins: { sort: { disable: true } },
            }),
        ]);

        tableProps = table.createViewModel(columns);
        console.log('Full tableProps:', JSON.parse(JSON.stringify(tableProps)));
    }

    $: ({ headerRows, pageRows, tableAttrs, tableBodyAttrs, flatColumns, pluginStates, rows } = tableProps || {});

    $: ({ sortKeys } = pluginStates?.sort || {});
    $: ({ hiddenColumnIds } = pluginStates?.hide || {});
    $: ({ hasNextPage, hasPreviousPage, pageIndex } = pluginStates?.page || {});
    $: ({ filterValue } = pluginStates?.filter || {});
    $: ({ selectedDataIds } = pluginStates?.select || {});

    let hideForId = {};
    $: if (flatColumns) {
        hideForId = Object.fromEntries(flatColumns.map(c => [c.id, true]));
    }

    $: if (hiddenColumnIds) {
        $hiddenColumnIds = Object.entries(hideForId)
            .filter(([, hide]) => !hide)
            .map(([id]) => id);
    }

    const hideableCols = ["username", "email", "is_admin", "is_active", "created_at"];
</script>

<div class="w-full">
    <div class="mb-4 flex items-center gap-4">
        <Input
            class="max-w-sm"
            placeholder="Filter users..."
            type="text"
            bind:value={$filterValue}
        />
        <DropdownMenu.Root>
            <DropdownMenu.Trigger asChild let:builder>
                <Button variant="outline" class="ml-auto" builders={[builder]}>
                    Columns <ChevronDown class="ml-2 h-4 w-4" />
                </Button>
            </DropdownMenu.Trigger>
            <DropdownMenu.Content>
                {#each flatColumns || [] as col}
                    {#if hideableCols.includes(col.id)}
                        <DropdownMenu.CheckboxItem bind:checked={hideForId[col.id]}>
                            {col.header}
                        </DropdownMenu.CheckboxItem>
                    {/if}
                {/each}
            </DropdownMenu.Content>
        </DropdownMenu.Root>
    </div>
    {#if loading}
        <p>Loading users...</p>
    {:else if error}
        <p class="text-red-500">Error: {error}</p>
    {:else if tableProps}
        <div class="rounded-md border">
            <Table.Root {...$tableAttrs}>
                <Table.Header>
                    {#each $headerRows as headerRow}
                        <Subscribe rowAttrs={headerRow.attrs()}>
                            <Table.Row>
                                {#each headerRow.cells as cell (cell.id)}
                                    <Subscribe
                                        attrs={cell.attrs()}
                                        let:attrs
                                        props={cell.props()}
                                        let:props
                                    >
                                        <Table.Head
                                            {...attrs}
                                            class={cn("[&:has([role=checkbox])]:pl-3")}
                                        >
                                            {#if cell.id === "email"}
                                                <Button variant="ghost" on:click={props.sort.toggle}>
                                                    <Render of={cell.render()} />
                                                    {#if $sortKeys[0]?.id === cell.id}
                                                        {#if $sortKeys[0].desc}
                                                            <ChevronDown class="ml-2 h-4 w-4 text-foreground" />
                                                        {:else}
                                                            <ChevronUp class="ml-2 h-4 w-4 text-foreground" />
                                                        {/if}
                                                    {:else}
                                                        <ChevronUp class="ml-2 h-4 w-4 text-muted-foreground opacity-0 group-hover:opacity-100" />
                                                    {/if}
                                                </Button>
                                            {:else}
                                                <Render of={cell.render()} />
                                            {/if}
                                        </Table.Head>
                                    </Subscribe>
                                {/each}
                            </Table.Row>
                        </Subscribe>
                    {/each}
                </Table.Header>
                <Table.Body {...$tableBodyAttrs}>
                    {#each $pageRows as row (row.id)}
                        <Subscribe rowAttrs={row.attrs()} let:rowAttrs>
                            <Table.Row
                                {...rowAttrs}
                                data-state={$selectedDataIds[row.id] && "selected"}
                            >
                                {#each row.cells as cell (cell.id)}
                                    <Subscribe attrs={cell.attrs()} let:attrs>
                                        <Table.Cell class="[&:has([role=checkbox])]:pl-3" {...attrs}>
                                            <Render of={cell.render()} />
                                        </Table.Cell>
                                    </Subscribe>
                                {/each}
                            </Table.Row>
                        </Subscribe>
                    {/each}
                </Table.Body>
            </Table.Root>
        </div>
        <div class="flex items-center justify-end space-x-2 py-4">
            <div class="text-muted-foreground flex-1 text-sm">
                {Object.keys($selectedDataIds).length} of {$rows.length} row(s) selected.
            </div>
            <Button
                variant="outline"
                size="sm"
                on:click={() => ($pageIndex = $pageIndex - 1)}
                disabled={!$hasPreviousPage}>Previous</Button
            >
            <Button
                variant="outline"
                size="sm"
                disabled={!$hasNextPage}
                on:click={() => ($pageIndex = $pageIndex + 1)}>Next</Button
            >
        </div>
    {:else}
        <p>No data available</p>
    {/if}
</div>