<script>
 import { Button } from "$lib/components/ui/button/index.js";
 import * as Card from "$lib/components/ui/card/index.js";
 
 export let listing;
</script>

<div class="w-full">
  <Card.Root class="h-full">
    <Card.Header>
      <Card.Title>{listing.title}</Card.Title>
      <Card.Description>{listing.listing_type}</Card.Description>
    </Card.Header>
    <Card.Content>
      <div class="grid w-full items-center gap-4">
        <div class="flex flex-col space-y-1.5">
          <p><strong>Location:</strong> {listing.city}, {listing.state}, {listing.country}</p>
        </div>
        {#if listing.price}
          <div class="flex flex-col space-y-1.5">
            <p><strong>Price:</strong> ${listing.price} {listing.price_type || ''}</p>
          </div>
        {/if}
        {#if listing.neighborhood}
          <div class="flex flex-col space-y-1.5">
            <p><strong>Neighborhood:</strong> {listing.neighborhood}</p>
          </div>
        {/if}
      </div>
    </Card.Content>
    <Card.Footer class="flex justify-between items-center">
      <span class="text-sm text-gray-500">Status: {listing.status}</span>
      <Button variant="outline" href="/listing/{listing.id}">View Details</Button>
    </Card.Footer>
  </Card.Root>
</div>
