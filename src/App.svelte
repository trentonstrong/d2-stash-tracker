<script lang="ts">
  import Import from './lib/Import.svelte'
</script>

<main class="container">
  <h1>D2R Stash Tracker</h1>

  <!-- <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div> -->

  <p>
    Select a Character file (.d2s) or Shared Stash file (.d2i) to import your stash.
  </p>

  <div class="row">
    <Import />
  </div>


</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
