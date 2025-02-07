<script>
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { api } from '$lib/api';
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Card, CardContent, CardHeader, CardTitle, CardDescription, CardFooter } from "$lib/components/ui/card";
  import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "$lib/components/ui/table";
  import { Switch } from "$lib/components/ui/switch";
  import { ArrowLeft, Customers, ListChecks, DollarSign, BarChart2, Download } from 'lucide-svelte';
  import { goto } from '$app/navigation';
  import { formatDate } from '$lib/utils';

  let customerData = null;
  let customer = null;
  let customerAccounts = [];
  let profiles = [];
  let directories = [];
  let loginHistory = []; // New variable for login history
  let loading = true;
  let error = null;
  let editing = false;
  let showDeactivatePrompt = false;

  $: customerId = $page.params.customerId;
  $: queryParams = $page.url.searchParams;

  onMount(() => {
    if (queryParams.get('edit') === 'true') {
      editing = true;
    }
    if (queryParams.get('deactivate') === 'true') {
      showDeactivatePrompt = true;
    }
  });

  $: if (customerId) {
    loadCustomerData(customerId);
  }

  async function loadCustomerData(id) {
    loading = true;
    try {
      const response = await api.admin.fetchCustomerById(id);
      console.log("Customer data:", response);
      
      // Destructure the response, providing default values
      ({ customer = null, customer_accounts: customerAccounts = [], profiles = [], directories = [], login_history: loginHistory = [] } = response);
      customerData = response;  // Keep the full response if needed elsewhere
    } catch (err) {
      error = err.message;
    } finally {
      loading = false;
    }
  }

  async function handleSave() {
    try {
      await api.admin.updateCustomer(customer.id, customer);
      editing = false;
    } catch (err) {
      error = err.message;
    }
  }

  async function handleToggleActive() {
    if (customer.is_active) {
      showDeactivatePrompt = true;
    } else {
      await toggleCustomerActive();
    }
  }

  async function toggleCustomerActive() {
    try {
      customer.is_active = !customer.is_active;
      await api.admin.updateCustomer(customer.id, { is_active: customer.is_active });
      showDeactivatePrompt = false;
    } catch (err) {
      error = err.message;
    }
  }

  async function handleResetPassword() {
    try {
      await api.admin.resetCustomerPassword(customer.id);
      alert("Password reset email sent to customer.");
    } catch (err) {
      error = err.message;
    }
  }

  function goBack() {
    goto('/customers');
  }

  function cancelDeactivation() {
    showDeactivatePrompt = false;
  }

  function formatDateTime(dateString) {
    return dateString ? formatDate(new Date(dateString), 'yyyy-MM-dd HH:mm:ss') : 'N/A';
  }
</script>

<svelte:head>
  <title>{customer ? customer.customername : 'Loading...'} | Customer Details</title>
</svelte:head>

<div class="container mx-auto px-4 py-8">
  <div class="mb-6">
    <Button variant="outline" on:click={goBack} class="flex items-center">
      <ArrowLeft class="mr-2 h-4 w-4" />
      Back to Customers
    </Button>
  </div>

  {#if loading}
    <p class="text-center text-xl">Loading customer details...</p>
  {:else if error}
    <p class="text-center text-xl text-red-500">Error: {error}</p>
  {:else if customer}
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <Card class="md:col-span-2">
        <CardHeader>
          <CardTitle class="text-2xl">{editing ? 'Edit Customer' : customer.customername}</CardTitle>
          <CardDescription>Customer ID: {customer.id}</CardDescription>
        </CardHeader>
        <CardContent>
          {#if editing}
            <form on:submit|preventDefault={handleSave}>
              <div class="grid gap-4">
                <div class="grid gap-2">
                  <Label for="customername">Customername</Label>
                  <Input id="customername" bind:value={customer.customername} required />
                </div>
                <div class="grid gap-2">
                  <Label for="email">Email</Label>
                  <Input id="email" type="email" bind:value={customer.email} required />
                </div>
                <div class="grid gap-2">
                  <Label for="phone">Phone</Label>
                  <Input id="phone" type="tel" bind:value={customer.phone} />
                </div>
              </div>
            </form>
          {:else}
            <div class="grid gap-4">
              <div class="flex justify-between items-center">
                <span class="font-semibold">Name:</span>
                <span>{customer.name}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Customer Type:</span>
                <span>{customer.customer_type}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Email:</span>
                <span>{customer.email || 'N/A'}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Phone:</span>
                <span>{customer.phone || 'N/A'}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">WhatsApp:</span>
                <span>{customer.whatsapp || 'N/A'}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Telegram:</span>
                <span>{customer.telegram || 'N/A'}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Social Media:</span>
                <div class="flex gap-2">
                  {#if customer.twitter}<span>Twitter</span>{/if}
                  {#if customer.instagram}<span>Instagram</span>{/if}
                  {#if customer.facebook}<span>Facebook</span>{/if}
                </div>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Website:</span>
                <span>{customer.website || 'N/A'}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Tax Info:</span>
                <div class="flex gap-2">
                  {#if customer.cpf}<span>CPF: {customer.cpf}</span>{/if}
                  {#if customer.cnpj}<span>CNPJ: {customer.cnpj}</span>{/if}
                  {#if customer.tin}<span>TIN: {customer.tin}</span>{/if}
                </div>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Annual Revenue:</span>
                <span>{customer.annual_revenue ? `$${customer.annual_revenue.toLocaleString()}` : 'N/A'}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Employee Count:</span>
                <span>{customer.employee_count || 'N/A'}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Created At:</span>
                <span>{new Date(customer.created_at).toLocaleString()}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Updated At:</span>
                <span>{new Date(customer.updated_at).toLocaleString()}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Active:</span>
                <Switch id="active" checked={customer.is_active} on:change={handleToggleActive} />
              </div>
              
              {#if customer.billing_address}
                <div class="mt-4">
                  <h3 class="font-semibold mb-2">Billing Address:</h3>
                  <div class="pl-4">
                    <p>{customer.billing_address.street}</p>
                    <p>{customer.billing_address.city}, {customer.billing_address.state} {customer.billing_address.postal_code}</p>
                    <p>{customer.billing_address.country}</p>
                  </div>
                </div>
              {/if}
              
              {#if customer.shipping_address}
                <div class="mt-4">
                  <h3 class="font-semibold mb-2">Shipping Address:</h3>
                  <div class="pl-4">
                    <p>{customer.shipping_address.street}</p>
                    <p>{customer.shipping_address.city}, {customer.shipping_address.state} {customer.shipping_address.postal_code}</p>
                    <p>{customer.shipping_address.country}</p>
                  </div>
                </div>
              {/if}
              
              <div class="mt-4">
                <h3 class="font-semibold mb-2">Customer Attributes:</h3>
                <div class="grid grid-cols-2 gap-2">
                  {#each Object.entries(customer.attributes) as [key, value]}
                    {#if value}
                      <div class="flex items-center gap-2">
                        <span class="text-sm">{key.replace(/_/g, ' ')}</span>
                      </div>
                    {/if}
                  {/each}
                </div>
              </div>
            </div>
          {/if}
        </CardContent>
        <CardFooter class="flex justify-between">
          <Button variant="outline" on:click={() => editing = !editing}>
            {editing ? 'Cancel' : 'Edit'}
          </Button>
          {#if editing}
            <Button on:click={handleSave}>Save Changes</Button>
          {:else}
            <Button variant="destructive" on:click={handleResetPassword}>Reset Password</Button>
          {/if}
        </CardFooter>
      </Card>

      <div class="space-y-6">
        <Card>
          <CardHeader>
            <CardTitle>Customer Accounts</CardTitle>
          </CardHeader>
          <CardContent>
            {#if customerAccounts.length > 0}
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Account ID</TableHead>
                    <TableHead>Customer ID</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {#each customerAccounts as account}
                    <TableRow>
                      <TableCell>{account.account_id}</TableCell>
                      <TableCell>{account.customer_id}</TableCell>
                    </TableRow>
                  {/each}
                </TableBody>
              </Table>
            {:else}
              <p>No customer accounts found.</p>
            {/if}
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Profiles</CardTitle>
          </CardHeader>
          <CardContent>
            {#if profiles.length > 0}
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Profile ID</TableHead>
                    <TableHead>Account ID</TableHead>
                    <TableHead>Directory ID</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {#each profiles as profile}
                    <TableRow>
                      <TableCell>{profile.id}</TableCell>
                      <TableCell>{profile.account_id}</TableCell>
                      <TableCell>{profile.directory_id}</TableCell>
                    </TableRow>
                  {/each}
                </TableBody>
              </Table>
            {:else}
              <p>No profiles found.</p>
            {/if}
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Directories</CardTitle>
          </CardHeader>
          <CardContent>
            {#if directories.length > 0}
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Directory ID</TableHead>
                    <TableHead>Name</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {#each directories as directory}
                    <TableRow>
                      <TableCell>{directory.id}</TableCell>
                      <TableCell>{directory.name}</TableCell>
                    </TableRow>
                  {/each}
                </TableBody>
              </Table>
            {:else}
              <p>No directories found.</p>
            {/if}
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Login History</CardTitle>
          </CardHeader>
          <CardContent>
            {#if loginHistory.length > 0}
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Date</TableHead>
                    <TableHead>IP Address</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {#each loginHistory as login}
                    <TableRow>
                      <TableCell>{formatDateTime(login.created_at)}</TableCell>
                      <TableCell>{login.ip_address}</TableCell>
                    </TableRow>
                  {/each}
                </TableBody>
              </Table>
            {:else}
              <p>No login history found.</p>
            {/if}
          </CardContent>
        </Card>
      </div>
    </div>
  {:else}
    <p class="text-center text-xl">Customer not found.</p>
  {/if}

  {#if showDeactivatePrompt}
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
      <Card class="w-96">
        <CardHeader>
          <CardTitle>Deactivate Customer</CardTitle>
        </CardHeader>
        <CardContent>
          <p>Are you sure you want to deactivate this customer?</p>
        </CardContent>
        <CardFooter class="flex justify-between">
          <Button variant="outline" on:click={cancelDeactivation}>Cancel</Button>
          <Button variant="destructive" on:click={toggleCustomerActive}>Deactivate</Button>
        </CardFooter>
      </Card>
    </div>
  {/if}
</div>