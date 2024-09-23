<script>
  import { onMount } from 'svelte';
  import { isAuthenticated } from '$lib/auth';
  import { fetchDashboardStats } from '$lib/api';
  import { Users, ListChecks, DollarSign, BarChart2, Tag } from 'lucide-svelte';
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import * as Tabs from "$lib/components/ui/tabs";
  import ChartComponent from '$lib/components/ChartComponent.svelte';

  let dashboardStats = null;
  let chartData = {
    labels: ['January', 'February', 'March', 'April', 'May', 'June', 'July'],
    datasets: [
      {
        label: 'Monthly Revenue',
        data: [0, 0, 0, 0, 0, 0, 0],
        backgroundColor: 'rgba(75, 192, 192, 0.6)',
        borderColor: 'rgb(75, 192, 192)',
        borderWidth: 1
      }
    ]
  };

  let chartOptions = {
    responsive: true,
    plugins: {
      legend: {
        position: 'top',
      },
      title: {
        display: true,
        text: 'Monthly Revenue'
      }
    }
  };

  async function loadDashboardStats() {
    if (!$isAuthenticated) return;
    try {
      dashboardStats = await fetchDashboardStats();
      // Update chart data with real data from dashboardStats
      // This is just an example, adjust according to your actual data structure
      chartData.datasets[0].data = dashboardStats.monthlyRevenue || chartData.datasets[0].data;
    } catch (error) {
      console.error('Failed to fetch dashboard stats:', error);
    }
  }

  onMount(async () => {
    if ($isAuthenticated) {
      await loadDashboardStats();
    }
  });
</script>

<svelte:head>
  <title>Oply Command Center</title>
</svelte:head>

{#if $isAuthenticated}
  <div class="space-y-8">
    <div class="flex items-center justify-between mb-8">
      <h2 class="text-3xl font-bold tracking-tight">Dashboard Overview</h2>
      <Button>
        <BarChart2 class="mr-2 h-4 w-4" />
        Generate Report
      </Button>
    </div>

    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4 mb-8">
      <Card.Root>
        <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
          <Card.Title class="text-sm font-medium">Total Users</Card.Title>
          <Users class="h-4 w-4 text-muted-foreground" />
        </Card.Header>
        <Card.Content>
          <div class="text-2xl font-bold">{dashboardStats?.totalUsers || '---'}</div>
        </Card.Content>
      </Card.Root>
      <Card.Root>
        <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
          <Card.Title class="text-sm font-medium">Active Listings</Card.Title>
          <ListChecks class="h-4 w-4 text-muted-foreground" />
        </Card.Header>
        <Card.Content>
          <div class="text-2xl font-bold">{dashboardStats?.activeListings || '---'}</div>
        </Card.Content>
      </Card.Root>
      <Card.Root>
        <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
          <Card.Title class="text-sm font-medium">Ad Sales</Card.Title>
          <DollarSign class="h-4 w-4 text-muted-foreground" />
        </Card.Header>
        <Card.Content>
          <div class="text-2xl font-bold">{dashboardStats?.adSales || '---'}</div>
        </Card.Content>
      </Card.Root>
      <Card.Root>
        <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
          <Card.Title class="text-sm font-medium">Total Categories</Card.Title>
          <Tag class="h-4 w-4 text-muted-foreground" />
        </Card.Header>
        <Card.Content>
          <div class="text-2xl font-bold">{dashboardStats?.totalCategories || '---'}</div>
        </Card.Content>
      </Card.Root>
    </div>

    <Card.Root>
      <Card.Header>
        <Card.Title>Monthly Revenue</Card.Title>
      </Card.Header>
      <Card.Content>
        <ChartComponent type="bar" data={chartData} options={chartOptions} />
      </Card.Content>
    </Card.Root>

    <Tabs.Root value="users" class="space-y-4">
      <Tabs.List>
        <Tabs.Trigger value="users">Users</Tabs.Trigger>
        <Tabs.Trigger value="listings">Listings</Tabs.Trigger>
        <Tabs.Trigger value="ad-purchases">Ad Purchases</Tabs.Trigger>
        <Tabs.Trigger value="categories">Categories</Tabs.Trigger>
      </Tabs.List>
      <Tabs.Content value="users" class="space-y-4">
        <h3 class="text-xl font-semibold">User Management</h3>
        <!-- Add user management content here -->
      </Tabs.Content>
      <Tabs.Content value="listings" class="space-y-4">
        <h3 class="text-xl font-semibold">Listing Management</h3>
        <!-- Add listing management content here -->
      </Tabs.Content>
      <Tabs.Content value="ad-purchases" class="space-y-4">
        <h3 class="text-xl font-semibold">Ad Purchase Management</h3>
        <!-- Add ad purchase management content here -->
      </Tabs.Content>
      <Tabs.Content value="categories" class="space-y-4">
        <h3 class="text-xl font-semibold">Category Management</h3>
        <!-- Add category management content here -->
      </Tabs.Content>
    </Tabs.Root>
  </div>
{:else}
  <div class="flex items-center justify-center min-h-screen">
    <Card.Root class="w-[350px]">
      <Card.Header>
        <Card.Title>Welcome to Oply Command Center</Card.Title>
        <Card.Description>Please log in to access the dashboard.</Card.Description>
      </Card.Header>
    </Card.Root>
  </div>
{/if}