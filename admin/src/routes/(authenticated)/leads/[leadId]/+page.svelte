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
  import { ArrowLeft, Leads, ListChecks, DollarSign, BarChart2, Download } from 'lucide-svelte';
  import { goto } from '$app/navigation';
  import { formatDate } from '$lib/utils';

  let leadData = null;
  let lead = null;
  let leadAccounts = [];
  let profiles = [];
  let directories = [];
  let loginHistory = []; // New variable for login history
  let loading = true;
  let error = null;
  let editing = false;
  let showDeactivatePrompt = false;

  $: leadId = $page.params.leadId;
  $: queryParams = $page.url.searchParams;

  onMount(() => {
    if (queryParams.get('edit') === 'true') {
      editing = true;
    }
    if (queryParams.get('deactivate') === 'true') {
      showDeactivatePrompt = true;
    }
  });

  $: if (leadId) {
    loadLeadData(leadId);
  }

  async function loadLeadData(id) {
    loading = true;
    try {
      const response = await api.admin.fetchLeadById(id);
      console.log("Lead data:", response);
      
      // Destructure the response, providing default values
      ({ lead = null, lead_accounts: leadAccounts = [], profiles = [], directories = [], login_history: loginHistory = [] } = response);
      leadData = response;  // Keep the full response if needed elsewhere
    } catch (err) {
      error = err.message;
    } finally {
      loading = false;
    }
  }

  async function handleSave() {
    try {
      await api.admin.updateLead(lead.id, lead);
      editing = false;
    } catch (err) {
      error = err.message;
    }
  }

  async function handleToggleActive() {
    if (lead.is_active) {
      showDeactivatePrompt = true;
    } else {
      await toggleLeadActive();
    }
  }

  async function toggleLeadActive() {
    try {
      lead.is_active = !lead.is_active;
      await api.admin.updateLead(lead.id, { is_active: lead.is_active });
      showDeactivatePrompt = false;
    } catch (err) {
      error = err.message;
    }
  }

  async function handleResetPassword() {
    try {
      await api.admin.resetLeadPassword(lead.id);
      alert("Password reset email sent to lead.");
    } catch (err) {
      error = err.message;
    }
  }

  function goBack() {
    goto('/leads');
  }

  function cancelDeactivation() {
    showDeactivatePrompt = false;
  }

  function formatDateTime(dateString) {
    return dateString ? formatDate(new Date(dateString), 'yyyy-MM-dd HH:mm:ss') : 'N/A';
  }
</script>

<svelte:head>
  <title>{lead ? lead.leadname : 'Loading...'} | Lead Details</title>
</svelte:head>

<div class="container mx-auto px-4 py-8">
  <div class="mb-6">
    <Button variant="outline" on:click={goBack} class="flex items-center">
      <ArrowLeft class="mr-2 h-4 w-4" />
      Back to Leads
    </Button>
  </div>

  {#if loading}
    <p class="text-center text-xl">Loading lead details...</p>
  {:else if error}
    <p class="text-center text-xl text-red-500">Error: {error}</p>
  {:else if lead}
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <Card class="md:col-span-2">
        <CardHeader>
          <CardTitle class="text-2xl">{editing ? 'Edit Lead' : lead.leadname}</CardTitle>
          <CardDescription>Lead ID: {lead.id}</CardDescription>
        </CardHeader>
        <CardContent>
          {#if editing}
            <form on:submit|preventDefault={handleSave}>
              <div class="grid gap-4">
                <div class="grid gap-2">
                  <Label for="leadname">Leadname</Label>
                  <Input id="leadname" bind:value={lead.leadname} required />
                </div>
                <div class="grid gap-2">
                  <Label for="email">Email</Label>
                  <Input id="email" type="email" bind:value={lead.email} required />
                </div>
                <div class="grid gap-2">
                  <Label for="phone">Phone</Label>
                  <Input id="phone" type="tel" bind:value={lead.phone} />
                </div>
              </div>
            </form>
          {:else}
            <div class="grid gap-4">
              <div class="flex justify-between items-center">
                <span class="font-semibold">Leadname:</span>
                <span>{lead.leadname}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Email:</span>
                <span>{lead.email}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Phone:</span>
                <span>{lead.phone}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Last Login:</span>
                <span>{new Date(lead.lastLogin).toLocaleString()}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Created At:</span>
                <span>{new Date(lead.created_at).toLocaleString()}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Updated At:</span>
                <span>{new Date(lead.updated_at).toLocaleString()}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Active Account:</span>
                <Switch id="active" checked={lead.is_active} on:change={handleToggleActive} />
              </div>
              <div class="flex justify-between items-center">
                <span class="font-semibold">Admin Account:</span>
                <Switch id="admin" checked={lead.is_admin} disabled />
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
            <CardTitle>Lead Accounts</CardTitle>
          </CardHeader>
          <CardContent>
            {#if leadAccounts.length > 0}
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Account ID</TableHead>
                    <TableHead>Lead ID</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {#each leadAccounts as account}
                    <TableRow>
                      <TableCell>{account.account_id}</TableCell>
                      <TableCell>{account.lead_id}</TableCell>
                    </TableRow>
                  {/each}
                </TableBody>
              </Table>
            {:else}
              <p>No lead accounts found.</p>
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
    <p class="text-center text-xl">Lead not found.</p>
  {/if}

  {#if showDeactivatePrompt}
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
      <Card class="w-96">
        <CardHeader>
          <CardTitle>Deactivate Lead</CardTitle>
        </CardHeader>
        <CardContent>
          <p>Are you sure you want to deactivate this lead?</p>
        </CardContent>
        <CardFooter class="flex justify-between">
          <Button variant="outline" on:click={cancelDeactivation}>Cancel</Button>
          <Button variant="destructive" on:click={toggleLeadActive}>Deactivate</Button>
        </CardFooter>
      </Card>
    </div>
  {/if}
</div>