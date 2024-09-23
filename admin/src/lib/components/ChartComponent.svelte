<script>
    import { onMount, onDestroy } from 'svelte';
    import Chart from 'chart.js/auto';
  
    export let type = 'bar';
    export let data = {};
    export let options = {};
  
    let canvas;
    let chart;
  
    onMount(() => {
      chart = new Chart(canvas, {
        type,
        data,
        options
      });
    });
  
    onDestroy(() => {
      if (chart) chart.destroy();
    });
  
    $: if (chart) {
      chart.data = data;
      chart.options = options;
      chart.update();
    }
  </script>
  
  <canvas bind:this={canvas}></canvas>