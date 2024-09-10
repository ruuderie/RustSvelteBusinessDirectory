<script>
  export let business;
</script>

<div class="business-card">
  <h2>{business.name}</h2>
  <p>Category: {business.category}</p>
  <p>Address: {business.address}</p>
  <a href="/business/{business.id}">View Details</a>
</div>

<style>
  .business-card {
    background-color: #f0f0f0;
    border-radius: 8px;
    padding: 20px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  h2 {
    margin-top: 0;
    margin-bottom: 10px;
  }

  p {
    margin-bottom: 5px;
  }

  a {
    display: inline-block;
    margin-top: 10px;
    text-decoration: none;
    color: #007bff;
  }
</style>
