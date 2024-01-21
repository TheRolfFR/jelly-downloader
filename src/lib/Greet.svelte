<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  import { Column, FormGroup, Grid, Row, TextInput } from "carbon-components-svelte";
  import { Button } from "carbon-components-svelte";

  let name = "";
  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name })
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <Grid>
      <Row>
        <Column noGutter><TextInput placeholder="Enter a name..." bind:value={name} /></Column>
        <Column noGutter class="btnCol"><Button size="field" type="submit" kind="secondary">Greet</Button></Column>
      </Row>
    </Grid>
  </form>
  <p>{greetMsg}</p>
</div>

<style>
  :global(#fields > *:first-child) {
    border: 5px solid red;
  }
  :global(.btnCol) {
    flex-shrink: 1;
    flex-grow: 0;
    padding-left: 0.6rem;
  }
  :global(.bx--btn--field) {
    padding-right: 12px;
  }
  p:not(:empty) {
    margin-top: 0.6rem;
  }
</style>