<script lang="ts">
  import { invoke } from '@tauri-apps/api'
  import * as d2s from './d2s/d2s'
  let character: d2s.types.ID2S;
  let files: FileList;
  let status = "";

  async function parseAndImport(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    // greetMsg = await invoke("greet", { name })
    console.log("files", files);

    if (files.length > 0) {
      const file = files[0];
      console.log("path", file.webkitRelativePath);
      const buffer = await file.arrayBuffer();
      const byteBuffer = new Uint8Array(buffer);
      const d2sFile = await d2s.read(byteBuffer);
      console.log("d2sFile", d2sFile);
      character = d2sFile;
      const characterJson = JSON.stringify(character);

      status = await invoke('import', { characterJson })
    }
  }

</script>

<div>
  <div class="row">
    <input id="import-input" placeholder="Choose a save to import" type="file" bind:files={files} />
    <button on:click={parseAndImport}>
      Import
    </button>
  </div>
  {#if !!character }
  <p>{character.header.name} Level {character.header.level} {character.header.class}</p>
  <p>Status: {status}</p>
  {:else}
  <p>No character loaded</p>
  {/if}
</div>
