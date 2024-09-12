<script>import "../app.css";
import Header from '$lib/components/Header.svelte';</script>

<Header></Header>

<main>
  <slot></slot>
</main>


<style>
  main {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
  }
</style>
