<script>
    import { goto } from '$app/navigation';
    import { api } from '$lib/api';
    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Card, CardContent, CardHeader, CardTitle, CardFooter } from "$lib/components/ui/card";
    import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "$lib/components/ui/select";
    import { Checkbox } from "$lib/components/ui/checkbox";
    import { ArrowLeft } from 'lucide-svelte';
    import { Textarea } from "$lib/components/ui/textarea";

    let newCustomer = {
        name: '',
        customer_type: 'BusinessEntity',
        attributes: {
            shipper: false,
            carrier: false,
            loan_seeker: false,
            loan_broker: false,
            software_vendor: false,
            tenant: false,
            software_development_client: false,
            salesforce_client: false,
            web3_client: false,
            bitcoiner: false,
            zk: false,
            lender: false,
            advertiser: false,
            gp: false,
            construction_contractor: false,
            construction_client: false,
            landlord: false
        },
        cpf: '',
        cnpj: '',
        tin: '',
        email: '',
        phone: '',
        whatsapp: '',
        telegram: '',
        twitter: '',
        instagram: '',
        facebook: '',
        website: '',
        annual_revenue: null,
        employee_count: null,
        billing_address: {
            street: '',
            city: '',
            state: '',
            postal_code: '',
            country: ''
        },
        shipping_address: {
            street: '',
            city: '',
            state: '',
            postal_code: '',
            country: ''
        }
    };

    let errorMessage = '';
    let sameAsShipping = true;

    const customerTypes = [
        { value: 'BusinessEntity', label: 'Business Entity' },
        { value: 'Person', label: 'Person' },
        { value: 'Household', label: 'Household' }
    ];

    $: if (sameAsShipping) {
        newCustomer.shipping_address = { ...newCustomer.billing_address };
    }

    async function handleCreateCustomer() {
        try {
            errorMessage = '';
            
            // Clean up empty strings in optional fields
            const cleanCustomer = { ...newCustomer };
            Object.keys(cleanCustomer).forEach(key => {
                if (cleanCustomer[key] === '') {
                    cleanCustomer[key] = null;
                }
            });

            // Convert numeric strings to numbers
            if (cleanCustomer.annual_revenue) {
                cleanCustomer.annual_revenue = parseFloat(cleanCustomer.annual_revenue);
            }
            if (cleanCustomer.employee_count) {
                cleanCustomer.employee_count = parseInt(cleanCustomer.employee_count);
            }

            await api.admin.createCustomer(cleanCustomer);
            goto('/customers');
        } catch (error) {
            console.error('Failed to create customer:', error);
            errorMessage = 'Failed to create customer. Please try again.';
        }
    }

    function handleBack() {
        goto('/customers');
    }
</script>

<div class="container mx-auto px-4 py-8">
    <Card>
        <CardHeader class="flex flex-row items-center justify-between">
            <div class="flex items-center space-x-4">
                <Button variant="ghost" on:click={handleBack}>
                    <ArrowLeft class="mr-2 h-4 w-4" />
                    Back
                </Button>
                <CardTitle>Create New Customer</CardTitle>
            </div>
        </CardHeader>
        <CardContent>
            <form on:submit|preventDefault={handleCreateCustomer} class="space-y-6">
                <!-- Basic Information -->
                <div class="space-y-4">
                    <h3 class="text-lg font-semibold">Basic Information</h3>
                    
                    <div class="grid gap-2">
                        <Label for="name">Name *</Label>
                        <Input id="name" bind:value={newCustomer.name} required />
                    </div>

                    <div class="grid gap-2">
                        <Label for="customer_type">Customer Type *</Label>
                        <Select bind:value={newCustomer.customer_type}>
                            <SelectTrigger>
                                <SelectValue placeholder="Select customer type" />
                            </SelectTrigger>
                            <SelectContent>
                                {#each customerTypes as type}
                                    <SelectItem value={type.value}>{type.label}</SelectItem>
                                {/each}
                            </SelectContent>
                        </Select>
                    </div>
                </div>

                <!-- Contact Information -->
                <div class="space-y-4">
                    <h3 class="text-lg font-semibold">Contact Information</h3>
                    
                    <div class="grid gap-4 md:grid-cols-2">
                        <div class="grid gap-2">
                            <Label for="email">Email</Label>
                            <Input id="email" type="email" bind:value={newCustomer.email} />
                        </div>
                        <div class="grid gap-2">
                            <Label for="phone">Phone</Label>
                            <Input id="phone" type="tel" bind:value={newCustomer.phone} />
                        </div>
                        <div class="grid gap-2">
                            <Label for="whatsapp">WhatsApp</Label>
                            <Input id="whatsapp" bind:value={newCustomer.whatsapp} />
                        </div>
                        <div class="grid gap-2">
                            <Label for="telegram">Telegram</Label>
                            <Input id="telegram" bind:value={newCustomer.telegram} />
                        </div>
                    </div>
                </div>

                <!-- Social Media -->
                <div class="space-y-4">
                    <h3 class="text-lg font-semibold">Social Media & Web</h3>
                    
                    <div class="grid gap-4 md:grid-cols-2">
                        <div class="grid gap-2">
                            <Label for="twitter">Twitter</Label>
                            <Input id="twitter" bind:value={newCustomer.twitter} />
                        </div>
                        <div class="grid gap-2">
                            <Label for="instagram">Instagram</Label>
                            <Input id="instagram" bind:value={newCustomer.instagram} />
                        </div>
                        <div class="grid gap-2">
                            <Label for="facebook">Facebook</Label>
                            <Input id="facebook" bind:value={newCustomer.facebook} />
                        </div>
                        <div class="grid gap-2">
                            <Label for="website">Website</Label>
                            <Input id="website" type="url" bind:value={newCustomer.website} />
                        </div>
                    </div>
                </div>

                <!-- Business Information -->
                <div class="space-y-4">
                    <h3 class="text-lg font-semibold">Business Information</h3>
                    
                    <div class="grid gap-4 md:grid-cols-3">
                        <div class="grid gap-2">
                            <Label for="cpf">CPF</Label>
                            <Input id="cpf" bind:value={newCustomer.cpf} />
                        </div>
                        <div class="grid gap-2">
                            <Label for="cnpj">CNPJ</Label>
                            <Input id="cnpj" bind:value={newCustomer.cnpj} />
                        </div>
                        <div class="grid gap-2">
                            <Label for="tin">TIN</Label>
                            <Input id="tin" bind:value={newCustomer.tin} />
                        </div>
                    </div>

                    <div class="grid gap-4 md:grid-cols-2">
                        <div class="grid gap-2">
                            <Label for="annual_revenue">Annual Revenue</Label>
                            <Input id="annual_revenue" type="number" bind:value={newCustomer.annual_revenue} />
                        </div>
                        <div class="grid gap-2">
                            <Label for="employee_count">Employee Count</Label>
                            <Input id="employee_count" type="number" bind:value={newCustomer.employee_count} />
                        </div>
                    </div>
                </div>

                <!-- Customer Attributes -->
                <div class="space-y-4">
                    <h3 class="text-lg font-semibold">Customer Attributes</h3>
                    
                    <div class="grid gap-4 md:grid-cols-3">
                        {#each Object.entries(newCustomer.attributes) as [key, value]}
                            <div class="flex items-center space-x-2">
                                <Checkbox 
                                    id={key} 
                                    bind:checked={newCustomer.attributes[key]}
                                />
                                <Label for={key}>{key.split('_').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join(' ')}</Label>
                            </div>
                        {/each}
                    </div>
                </div>

                <!-- Billing Address -->
                <div class="space-y-4">
                    <h3 class="text-lg font-semibold">Billing Address</h3>
                    
                    <div class="grid gap-4">
                        <div class="grid gap-2">
                            <Label for="billing_street">Street</Label>
                            <Input id="billing_street" bind:value={newCustomer.billing_address.street} />
                        </div>
                        <div class="grid gap-4 md:grid-cols-2">
                            <div class="grid gap-2">
                                <Label for="billing_city">City</Label>
                                <Input id="billing_city" bind:value={newCustomer.billing_address.city} />
                            </div>
                            <div class="grid gap-2">
                                <Label for="billing_state">State</Label>
                                <Input id="billing_state" bind:value={newCustomer.billing_address.state} />
                            </div>
                        </div>
                        <div class="grid gap-4 md:grid-cols-2">
                            <div class="grid gap-2">
                                <Label for="billing_postal_code">Postal Code</Label>
                                <Input id="billing_postal_code" bind:value={newCustomer.billing_address.postal_code} />
                            </div>
                            <div class="grid gap-2">
                                <Label for="billing_country">Country</Label>
                                <Input id="billing_country" bind:value={newCustomer.billing_address.country} />
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Shipping Address -->
                <div class="space-y-4">
                    <div class="flex items-center space-x-2">
                        <h3 class="text-lg font-semibold">Shipping Address</h3>
                        <Checkbox 
                            id="same_as_billing"
                            bind:checked={sameAsShipping}
                        />
                        <Label for="same_as_billing">Same as billing</Label>
                    </div>
                    
                    {#if !sameAsShipping}
                        <div class="grid gap-4">
                            <div class="grid gap-2">
                                <Label for="shipping_street">Street</Label>
                                <Input id="shipping_street" bind:value={newCustomer.shipping_address.street} />
                            </div>
                            <div class="grid gap-4 md:grid-cols-2">
                                <div class="grid gap-2">
                                    <Label for="shipping_city">City</Label>
                                    <Input id="shipping_city" bind:value={newCustomer.shipping_address.city} />
                                </div>
                                <div class="grid gap-2">
                                    <Label for="shipping_state">State</Label>
                                    <Input id="shipping_state" bind:value={newCustomer.shipping_address.state} />
                                </div>
                            </div>
                            <div class="grid gap-4 md:grid-cols-2">
                                <div class="grid gap-2">
                                    <Label for="shipping_postal_code">Postal Code</Label>
                                    <Input id="shipping_postal_code" bind:value={newCustomer.shipping_address.postal_code} />
                                </div>
                                <div class="grid gap-2">
                                    <Label for="shipping_country">Country</Label>
                                    <Input id="shipping_country" bind:value={newCustomer.shipping_address.country} />
                                </div>
                            </div>
                        </div>
                    {/if}
                </div>

                {#if errorMessage}
                    <p class="text-red-500 mt-2">{errorMessage}</p>
                {/if}
            </form>
        </CardContent>
        <CardFooter>
            <Button on:click={handleCreateCustomer}>Create Customer</Button>
        </CardFooter>
    </Card>
</div>