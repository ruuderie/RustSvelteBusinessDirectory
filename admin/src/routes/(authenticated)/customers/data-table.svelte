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
    import { ChevronDown } from 'lucide-svelte';
    import { Plus } from 'lucide-svelte';
    import { goto } from '$app/navigation';

    let customers = [];
    let loading = true;
    let error = null;

    const filterValue = writable('');

    let table;
    const tableProps = writable(null);
    const rows = writable([]);

    onMount(async () => {
        try {
            customers = await api.admin.fetchCustomers();
            loading = false;
            await initializeTable();
        } catch (err) {
            error = err.message;
            loading = false;
        }
    });

    async function initializeTable() {
        if (customers.length === 0) return;

        const data = readable(customers);

        table = createTable(data, {
            sort: addSortBy({ disableMultiSort: true }),
            page: addPagination({ initialPageSize: 10 }),
            filter: addTableFilter({
                fn: ({ filterValue, value }) => value?.toLowerCase().includes(filterValue.toLowerCase()),
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
                header: 'Type',
                accessor: 'customer_type',
            }),
            table.column({
                header: 'Email',
                accessor: 'email',
                cell: ({ value }) => value || 'N/A',
            }),
            table.column({
                header: 'Phone',
                accessor: 'phone',
                cell: ({ value }) => value || 'N/A',
            }),
            table.column({
                header: 'Annual Revenue',
                accessor: 'annual_revenue',
                cell: ({ value }) => value ? `$${value.toLocaleString()}` : 'N/A',
            }),
            table.column({
                header: 'Status',
                accessor: 'is_active',
                cell: ({ value }) => value ? 'Active' : 'Inactive',
            }),
            table.column({
                header: 'Created At',
                accessor: 'created_at',
                cell: ({ value }) => new Date(value).toLocaleString(),
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

    function handleCreateCustomer() {
        goto('/customers/new');
    }

    $: if (table && $filterValue) {
        table.filter.setGlobalFilter($filterValue);
    }

    const hideableCols = ["name", "customer_type", "email", "phone", "annual_revenue", "is_active", "created_at"];
    let hideForId = {};

    $: if ($tableProps?.flatColumns) {
        hideForId = Object.fromEntries($tableProps.flatColumns.map(c => [c.id, true]));
    }

    $: if ($tableProps?.pluginStates?.hide?.hiddenColumnIds) {
        $tableProps.pluginStates.hide.hiddenColumnIds = Object.entries(hideForId)
            .filter(([, hide]) => !hide)
            .map(([id]) => id);
    }
</script>

<div class="w-full">
    <div class="mb-4 flex items-center gap-4">
        <Input
            class="max-w-sm"
            placeholder="Filter customers..."
            type="text"
            bind:value={$filterValue}
        />
        <Button variant="outline" on:click={handleCreateCustomer} class="ml-auto">
            <Plus class="mr-2 h-4 w-4" />
            Create Customer
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
        <div class="flex justify-center items-center h-32">
            <span class="loading loading-spinner loading-lg"></span>
        </div>
    {:else if error}
        <div class="text-red-500 text-center">{error}</div>
    {:else if $tableProps}
        <div class="rounded-md border">
            <Table.Root>
                <Table.Header>
                    {#each $tableProps.headerRows as headerRow}
                        <Table.Row>
                            {#each headerRow.cells as cell (cell.id)}
                                <Table.Head class="whitespace-nowrap">
                                    {#if cell.props.sort}
                                        <button
                                            class="inline-flex items-center gap-1"
                                            on:click={() => cell.props.sort.toggle()}
                                        >
                                            {cell.props.title}
                                            {#if cell.props.sort.order === "asc"}
                                                ↑
                                            {:else if cell.props.sort.order === "desc"}
                                                ↓
                                            {/if}
                                        </button>
                                    {:else}
                                        {cell.props.title}
                                    {/if}
                                </Table.Head>
                            {/each}
                        </Table.Row>
                    {/each}
                </Table.Header>
                <Table.Body>
                    {#each $tableProps.rows as row (row.id)}
                        <Table.Row>
                            {#each row.cells as cell (cell.id)}
                                <Table.Cell>
                                    {#if cell.props.component}
                                        <svelte:component 
                                            this={cell.props.component} 
                                            {...cell.props.props}
                                        />
                                    {:else}
                                        {cell.props.value}
                                    {/if}
                                </Table.Cell>
                            {/each}
                        </Table.Row>
                    {/each}
                </Table.Body>
            </Table.Root>
        </div>

        <div class="flex items-center justify-end space-x-2 py-4">
            <div class="text-sm text-gray-600">
                Page {$tableProps.pageInfo.pageIndex + 1} of {$tableProps.pageInfo.pageCount}
            </div>
            <Button
                variant="outline"
                size="sm"
                on:click={() => $tableProps.pageInfo.previousPage()}
                disabled={!$tableProps.pageInfo.hasPreviousPage}
            >
                Previous
            </Button>
            <Button
                variant="outline"
                size="sm"
                on:click={() => $tableProps.pageInfo.nextPage()}
                disabled={!$tableProps.pageInfo.hasNextPage}
            >
                Next
            </Button>
        </div>
    {/if}
</div>