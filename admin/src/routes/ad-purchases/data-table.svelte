<script>

    // import data from api.js 
    import { fetchAdPurchases } from '$lib/api';
    import { readable } from "svelte/store";
  //  import DataTableActions from "./data-table-actions.svelte";
  //  import DataTableCheckbox from "./data-table/data-table-checkbox.svelte";
    import * as Table from "$lib/components/ui/table/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
    import { cn } from "$lib/utils.js";
    import { Input } from "$lib/components/ui/input/index.js";


   // import CaretSort from "svelte-radix/CaretSort.svelte";
  //import ChevronDown from "svelte-radix/ChevronDown.svelte";
  import {
    Render,
    Subscribe,
    createRender,
    createTable
  } from "svelte-headless-table";
  import {
    addHiddenColumns,
    addPagination,
    addSelectedRows,
    addSortBy,
    addTableFilter
  } from "svelte-headless-table/plugins";


  const fetchData = async () => {
    const data = await fetchAdPurchases();
    return data;
  };

  const data = fetchData();
  const table = createTable(readable(data));

  const columns = table.createColumns([
    table.column({
      header: "ID",
      accessor: "id"
    }),
    table.column({
      header: "Profile ID",
      accessor: "profile_id"
    }),
    table.column({
      header: "Listing ID",
      accessor: "listing_id"
    }),
    table.column({
      header: "Start Date",
      accessor: "start_date"
    }),
    table.column({
      header: "End Date",
      accessor: "end_date"
    }),
    table.column({
      header: "Price",
      accessor: "price",
      cell: ({ value }) => {
      const formatted = new Intl.NumberFormat("en-US", {
        style: "currency",
        currency: "USD",
      }).format(value);
        return formatted;
      },
    }),
    table.column({
      header: "Status",
      accessor: "status"
    }),
    table.column({
      header: "Actions",
      accessor: "id",
      cell: (row) => {
        return `<DataTableActions id="${row.value}" />`;
      }
    }),
  ]);
  const { headerRows, pageRows, tableAttrs, tableBodyAttrs } =
    table.createViewModel(columns);

</script>

<!--create table-->
<div class="rounded-md border">
    <Table.Root {...$tableAttrs}>
      <Table.Header>
        {#each $headerRows as headerRow}
          <Subscribe rowAttrs={headerRow.attrs()}>
            <Table.Row>
              {#each headerRow.cells as cell (cell.id)}
                <Subscribe attrs={cell.attrs()} let:attrs props={cell.props()}>
                  <Table.Head {...attrs}>
                    <Render of={cell.render()} />
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
            <Table.Row {...rowAttrs}>
              {#each row.cells as cell (cell.id)}
                <Subscribe attrs={cell.attrs()} let:attrs>
                  <Table.Cell {...attrs}>
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