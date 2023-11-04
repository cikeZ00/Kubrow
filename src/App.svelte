<script lang="ts">
  import Greet from './lib/Greet.svelte'
</script>

<main class="container">
  <h1>Kubrow</h1>
  <header>

  <a href="https://github.com/cikeZ00/Kubrow" target="_blank" rel="noreferrer">
    <img src="/Kubrow.svg" class="logo kubrow" alt="Kubrow Logo" />
  </a>
  <p>
    A warframe companion for both new and veteran players alike.
  </p>
  
</header>

  <div class="row">
    <Greet />
  </div>


</main>

<style>
  .logo.kubrow:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }
</style>