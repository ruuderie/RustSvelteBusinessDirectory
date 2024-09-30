<script>
    import { onMount } from 'svelte';
    import { writable, readable } from 'svelte/store';
    import { api } from '$lib/api';  // Update this import
    import DataTableActions from "./data-table-actions.svelte";
    import DataTableCheckbox from "./data-table-checkbox.svelte";
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
    import { ChevronDown, ChevronUp } from 'lucide-svelte';

    let users = [];
    let loading = true;
    let error = null;

    const filterValue = writable('');

    let table;
    const tableProps = writable(null);

    const rows = writable([]); // Ensure rows is a writable store

    onMount(async () => {
        try {
            users = await api.admin.fetchUsers();  // Update this line
            console.log('users', users);
            loading = false;
            await initializeTable();
            console.log('tableProps', $tableProps);
            console.log('tableProps.flatColumns', $tableProps.flatColumns);
            console.log('tableProps.flatColumns.length', $tableProps.flatColumns?.length);
            console.log('rows', $rows);
        } catch (err) {
            error = err.message;
            loading = false;
        }
    });

    async function initializeTable() {
        if (users.length === 0) return;

        const data = readable(users);

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
                header: 'Username',
                accessor: 'username',
            }),
            table.column({
                header: 'Email',
                accessor: 'email',
            }),
            table.column({
                header: 'Is Admin',
                accessor: 'is_admin',
                cell: ({ value }) => value ? 'Yes' : 'No',
            }),
            table.column({
                header: 'Is Active',
                accessor: 'is_active',
                cell: ({ value }) => value ? 'Yes' : 'No',
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

        console.log('Full tableProps after initialization:', JSON.parse(JSON.stringify($tableProps)));
        console.log('Rows structure:', $tableProps.rows);
        console.log('First row sample:', $tableProps.rows[0]);
    }

    $: if (table && $filterValue) {
        table.filter.setGlobalFilter($filterValue);
    }

    let hideForId = {};
    const flatColumns = [];

    function initializeHideForId() {
        hideForId = Object.fromEntries(flatColumns.map(c => [c.id, true]));
    }

    $: if ($tableProps) {
        console.log('tableProps', $tableProps);
        console.log('tableProps.flatColumns', $tableProps.flatColumns);
        console.log('tableProps.flatColumns.length', $tableProps.flatColumns?.length);
        console.log('rows', $rows);
    }

    $: if (tableProps?.pluginStates?.hide?.hiddenColumnIds) {
        tableProps.pluginStates.hide.hiddenColumnIds = Object.entries(hideForId)
            .filter(([, hide]) => !hide)
            .map(([id]) => id);
    }

    const hideableCols = ["username", "email", "is_admin", "is_active", "created_at"];

    $: if ($tableProps && $tableProps.rows && typeof $tableProps.rows.subscribe === 'function') {
        $tableProps.rows.subscribe(value => {
            rows.set(value);
        });
    }

    $: if ($rows && typeof $rows.subscribe === 'function') {
        $rows.subscribe(value => {
            if (!Array.isArray(value)) {
                console.error('rows is not an array:', value);
            }
        });
    }

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
            <DropdownMenu.Content class="bg-white shadow-md rounded-md">
                {#each tableProps?.flatColumns || [] as col}
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
        <p>Loading users...</p>
    {:else if error}
        <p class="text-red-500">Error: {error}</p>
    {:else if $tableProps && hideableCols?.length > 0}
        <div class="rounded-md border">
            <Table.Root {...($tableProps.tableAttrs || {})}>
                <Table.Header>
                    <Table.Row>
                        {#each $tableProps.flatColumns || [] as column (column.id)}
                            <Table.Head {...(column.headerAttrs || {})} class={cn("[&:has([role=checkbox])]:pl-3")}>
                                {#if column.id === "email"}
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
                        {:else}
                            <Table.Row>
                                <Table.Cell colspan={$tableProps.flatColumns?.length || 1}>No data available</Table.Cell>
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
        <div class="flex items-center justify-end space-x-2 py-4">
            <div class="text-muted-foreground flex-1 text-sm">
                {Object.keys($tableProps.pluginStates?.select?.selectedDataIds || {})?.length} of {tableProps.rows?.length || 0} row(s) selected.
            </div>
            <Button
                variant="outline"
                size="sm"
                on:click={() => ($tableProps.pluginStates.page.pageIndex = $tableProps.pluginStates.page.pageIndex - 1)}
                disabled={!$tableProps.pluginStates?.page?.hasPreviousPage}>Previous</Button
            >
            <Button
                variant="outline"
                size="sm"
                disabled={!$tableProps.pluginStates?.page?.hasNextPage}
                on:click={() => ($tableProps.pluginStates.page.pageIndex = $tableProps.pluginStates.page.pageIndex + 1)}>Next</Button
            >
        </div>
    {:else}
        <p>No data available or table not initialized</p>
    {/if}
</div>