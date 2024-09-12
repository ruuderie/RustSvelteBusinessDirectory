<script>
 import { Button } from "$lib/components/ui/button/index.js";
 import * as Card from "$lib/components/ui/card/index.js";
 
 export let business;
</script>

<div class="w-full">
  <Card.Root class="h-full">
    <Card.Header>
      <Card.Title>{business.name}</Card.Title>
      <Card.Description>{business.category}</Card.Description>
    </Card.Header>
    <Card.Content>
      <div class="grid w-full items-center gap-4">
        <div class="flex flex-col space-y-1.5">
          <p><strong>Address:</strong> {business.address}</p>
        </div>
        {#if business.phone}
          <div class="flex flex-col space-y-1.5">
            <p><strong>Phone:</strong> {business.phone}</p>
          </div>
        {/if}
      </div>
    </Card.Content>
    <Card.Footer class="flex justify-end">
      <Button variant="outline" href="/business/{business.id}">View Details</Button>
    </Card.Footer>
  </Card.Root>
</div>
