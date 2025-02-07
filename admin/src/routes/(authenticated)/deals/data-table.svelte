<script>
    import { onMount } from 'svelte';
    import { writable, readable } from 'svelte/store';
    import { api } from '$lib/api';
    import DataTableActions from "./data-table-actions.svelte";
    import * as Table from "$lib/components/ui/table";
    import { Button } from "$lib/components/ui/button";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { Input } from "$lib/components/ui/input";
    import { createTable } from "svelte-headless-table";
    import {
        addHiddenColumns,
        addPagination,
        addSelectedRows,
        addSortBy,
        addTableFilter,
    } from "svelte-headless-table/plugins";
    import { cn } from "$lib/utils.js";
    import { ChevronDown, ChevronUp, Plus } from 'lucide-svelte';
    import { goto } from '$app/navigation';

    let deals = [];
    let loading = true;
    let error = null;

    const filterValue = writable('');
    let table;
    const tableProps = writable(null);
    const rows = writable([]);

    onMount(async () => {
        try {
            deals = await api.admin.fetchDeals();
            loading = false;
            await initializeTable();
        } catch (err) {
            error = err.message;
            loading = false;
        }
    });

    async function initializeTable() {
        if (deals.length === 0) return;

        const data = readable(deals);

        table = createTable(data, {
            sort: addSortBy({ disableMultiSort: true }),
            page: addPagination({ initialPageSize: 10 }),
            filter: addTableFilter({
                fn: ({ filterValue, value }) => value.toLowerCase().includes(filterValue.toLowerCase()),
            }),
            select: addSelectedRows(),
            hide: addHiddenColumns(),
        });

        const columns = table.createColumns([
            table.column({
                header: 'Name',
                accessor: 'name',
            }),
            table.column({
                header: 'Customer',
                accessor: 'customer_id',
            }),
            table.column({
                header: 'Amount',
                accessor: 'amount',
                cell: ({ value }) => `$${value.toLocaleString()}`,
            }),
            table.column({
                header: 'Status',
                accessor: 'status',
            }),
            table.column({
                header: 'Stage',
                accessor: 'stage',
            }),
            table.column({
                header: 'Close Date',
                accessor: 'close_date',
                cell: ({ value }) => value ? new Date(value).toLocaleDateString() : 'N/A',
            }),
            table.column({
                header: 'Active',
                accessor: 'is_active',
                cell: ({ value }) => value ? 'Yes' : 'No',
            }),
            table.column({
                header: 'Actions',
                accessor: 'id',
                cell: ({ value }) => ({
                    component: DataTableActions,
                    props: { id: value }
                })
            }),
        ]);

        const viewModel = table.createViewModel(columns);
        tableProps.set(viewModel);
    }

    function handleCreateDeal() {
        goto('/deals/create');
    }

    $: if (table && $filterValue) {
        table.filter.setGlobalFilter($filterValue);
    }

    const hideableCols = ["name", "customer_id", "amount", "status", "stage", "close_date", "is_active"];
</script>

<div class="w-full">
    <div class="mb-4 flex items-center gap-4">
        <Input
            class="max-w-sm"
            placeholder="Filter deals..."
            type="text"
            bind:value={$filterValue}
        />
        <Button variant="outline" on:click={handleCreateDeal} class="ml-auto">
            <Plus class="mr-2 h-4 w-4" />
            Create Deal
        </Button>
        <DropdownMenu.Root>
            <DropdownMenu.Trigger asChild let:builder>
                <Button variant="outline" class="ml-auto" builders={[builder]}>
                    Columns <ChevronDown class="ml-2 h-4 w-4" />
                </Button>
            </DropdownMenu.Trigger>
            <DropdownMenu.Content class="bg-white shadow-md rounded-md">
                {#each $tableProps?.flatColumns || [] as col}
                    {#if hideableCols.includes(col.id)}
                        <DropdownMenu.CheckboxItem 
                            checked={hideForId[col.id]} 
                            onCheckedChange={(checked) => {
                                hideForId[col.id] = checked;
                                hideForId = {...hideForId};
                            }}
                        >
                            {col.header}
                        </DropdownMenu.CheckboxItem>
                    {/if}
                {/each}
            </DropdownMenu.Content>
        </DropdownMenu.Root>
    </div>

    {#if loading}
        <p>Loading deals...</p>
    {:else if error}
        <p class="text-red-500">Error: {error}</p>
    {:else if $tableProps}
        <div class="rounded-md border">
            <Table.Root {...($tableProps.tableAttrs || {})}>
                <Table.Header>
                    <Table.Row>
                        {#each $tableProps.flatColumns || [] as column (column.id)}
                            <Table.Head {...(column.headerAttrs || {})} class={cn("[&:has([role=checkbox])]:pl-3")}>
                                {#if column.id === "amount"}
                                    <Button variant="ghost" on:click={() => column.sort?.toggle?.()}>
                                        {column.header}
                                        {#if $tableProps.pluginStates?.sort?.sortKeys?.[0]?.id === column.id}
                                            {#if $tableProps.pluginStates.sort.sortKeys[0].desc}
                                                <ChevronDown class="ml-2 h-4 w-4 text-foreground" />
                                            {:else}
                                                <ChevronUp class="ml-2 h-4 w-4 text-foreground" />
                                            {/if}
                                        {:else}
                                            <ChevronUp class="ml-2 h-4 w-4 text-muted-foreground opacity-0 group-hover:opacity-100" />
                                        {/if}
                                    </Button>
                                {:else}
                                    {column.header}
                                {/if}
                            </Table.Head>
                        {/each}
                    </Table.Row>
                </Table.Header>
                <Table.Body {...($tableProps.tableBodyAttrs || {})}>
                    {#if Array.isArray($rows)}
                        {#each $rows || [] as row (row.id)}
                            <Table.Row {...(row.attrs?.() || {})} data-state={$tableProps.pluginStates?.select?.selectedDataIds?.[row.id] && "selected"}>
                                {#each row.cells as cell (cell.id)}
                                    <Table.Cell class="[&:has([role=checkbox])]:pl-3" {...(cell.attrs?.() || {})}>
                                        {#if typeof cell.render === 'function'}
                                            {#if cell.column.id === 'id'}
                                                {@const renderResult = cell.render()}
                                                {#if renderResult && renderResult.component}
                                                    <svelte:component this={renderResult.component} {...renderResult.props} />
                                                {:else}
                                                    {cell.value ?? ''}
                                                {/if}
                                            {:else}
                                                {cell.render()}
                                            {/if}
                                        {:else}
                                            {cell.value ?? ''}
                                        {/if}
                                    </Table.Cell>
                                {/each}
                            </Table.Row>
                        {/each}
                    {:else}
                        <Table.Row>
                            <Table.Cell colspan={$tableProps.flatColumns?.length || 1}>No data available</Table.Cell>
                        </Table.Row>
                    {/if}
                </Table.Body>
            </Table.Root>
        </div>
    {:else}
        <p>No data available or table not initialized</p>
    {/if}
</div>