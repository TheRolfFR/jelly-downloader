<script lang="ts">
  import 'carbon-components-svelte/css/all.css'
  import Greet from './lib/Greet.svelte'
  import { Header, SkipToContent, Content, ImageLoader, Button } from 'carbon-components-svelte'
  import { invoke } from '@tauri-apps/api/core'

  let theme = 'g90' // "white" | "g10" | "g80" | "g90" | "g100"

  $: document.documentElement.setAttribute('theme', theme)

  function openDialog() {
    invoke("dialog")
  }
</script>

<Header>
  <svelte:fragment slot="platform">
    <div>Tauri with Carbon Svelte</div>
  </svelte:fragment>
  <svelte:fragment slot="skip-to-content">
    <SkipToContent />
  </svelte:fragment>
</Header>

<Content id="main-content">
  <h1>Welcome to Tauri!</h1>
  <div id="logos">
    <div><ImageLoader style="filter: drop-shadow(0 0 var(--drop-shadow-size) #747bff)" src="/vite.svg" /></div>
    <div><ImageLoader style="filter: drop-shadow(0 0 var(--drop-shadow-size) #24c8db)" src="/tauri.svg" /></div>
    <div><ImageLoader style="filter: drop-shadow(0 0 var(--drop-shadow-size) #ff3e00)" src="/svelte.svg" /></div>
    <div><ImageLoader style="filter: drop-shadow(0 0 var(--drop-shadow-size) #42AAEA)" src="/carbon.png" /></div>
  </div>
  <div><Greet /></div>
  <div><Button kind="tertiary" size="small" id="open" on:click={openDialog}>Open dialog</Button></div>
</Content>

<style lang="scss">
  $drop_shadow_size: 0.7em;
  :root {
    --drop-shadow-size: 0.7em;
  }

  :global(#main-content) {
    text-align: center;
    padding: 1.2rem;
    width: 100vw;
    overflow: hidden;

    & > * + * {
      margin-top: 1.2rem;
    }
  }

  :global(#open) {
    width: 100%;
    max-width: none;
    display: block;
    padding: 12px;
    text-align: center;
  }

  #logos {
    margin: calc($drop_shadow_size + 1.2rem) 0;
    display: grid;
    grid-auto-flow: column;
    grid-auto-columns: 1fr;
    gap: 1.2rem;

    & :global(>*) {
      display: flex;
      align-items: center;
    }
  }
</style>
