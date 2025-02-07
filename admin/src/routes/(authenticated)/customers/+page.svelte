<script>
  import { onMount } from 'svelte';
  import { api } from '$lib/api';
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "$lib/components/ui/table";
  import { Plus, Search } from 'lucide-svelte';
  import { goto } from '$app/navigation';

  let customers = [];
  let loading = true;
  let error = null;
  let currentPage = 1;
  let totalPages = 1;
  let searchQuery = '';

  async function loadCustomers(page = 1) {
    loading = true;
    try {
      const response = await api.admin.fetchCustomers(page);
      customers = response.customers;
      totalPages = response.total_pages;
      currentPage = response.current_page;
    } catch (err) {
      error = err.message;
    } finally {
      loading = false;
    }
  }

  function handleCustomerClick(customerId) {
    goto(`/customers/${customerId}`);
  }

  function handleCreateCustomer() {
    goto('/customers/new');
  }

  onMount(() => {
    loadCustomers();
  });
</script>

<div class="container mx-auto px-4 py-8">
  <div class="flex justify-between items-center mb-6">
    <h1 class="text-2xl font-bold">Customers</h1>
    <Button on:click={handleCreateCustomer}>
      <Plus class="mr-2 h-4 w-4" />
      New Customer
    </Button>
  </div>

  <div class="mb-6">
    <div class="relative">
      <Search class="absolute left-3 top-3 h-4 w-4 text-gray-400" />
      <Input
        type="search"
        placeholder="Search customers..."
        class="pl-10"
        bind:value={searchQuery}
      />
    </div>
  </div>

  {#if loading}
    <p class="text-center text-xl">Loading customers...</p>
  {:else if error}
    <p class="text-center text-xl text-red-500">Error: {error}</p>
  {:else}
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>Name</TableHead>
          <TableHead>Email</TableHead>
          <TableHead>Phone</TableHead>
          <TableHead>Type</TableHead>
          <TableHead>Status</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {#each customers as customer}
          <TableRow 
            class="cursor-pointer hover:bg-gray-50" 
            on:click={() => handleCustomerClick(customer.id)}
          >
            <TableCell>{customer.name}</TableCell>
            <TableCell>{customer.email || 'N/A'}</TableCell>
            <TableCell>{customer.phone || 'N/A'}</TableCell>
            <TableCell>{customer.customer_type}</TableCell>
            <TableCell>
              <span class={`px-2 py-1 rounded-full text-xs ${customer.is_active ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'}`}>
                {customer.is_active ? 'Active' : 'Inactive'}
              </span>
            </TableCell>
          </TableRow>
        {/each}
      </TableBody>
    </Table>

    <div class="mt-4 flex justify-center gap-2">
      <Button 
        variant="outline" 
        disabled={currentPage === 1}
        on:click={() => loadCustomers(currentPage - 1)}
      >
        Previous
      </Button>
      <span class="py-2 px-4">
        Page {currentPage} of {totalPages}
      </span>
      <Button 
        variant="outline" 
        disabled={currentPage === totalPages}
        on:click={() => loadCustomers(currentPage + 1)}
      >
        Next
      </Button>
    </div>
  {/if}
</div>