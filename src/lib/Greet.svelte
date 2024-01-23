<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  import { TextInput } from "carbon-components-svelte";
  import { Button } from "carbon-components-svelte";
  // @ts-ignore
  import Download from "carbon-icons-svelte/lib/Download.svelte";

  let url = "";
  let downloadMsg = ""

  async function download(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    downloadMsg = await invoke("download", { url })
  }
</script>

<div>
  <form on:submit|preventDefault={download}>
    <div id="url-input">
      <TextInput placeholder="Enter URL" bind:value={url} />
    </div>
    <Button size="field" type="submit" kind="primary" icon={Download} />
  </form>
  <p>{downloadMsg}</p>
</div>

<style lang="scss">
  form {
    display: flex;
    gap: 0.6rem;

    #url-input {
      flex-grow: 1;
    }
  }
  p:not(:empty) {
    margin-top: 0.6rem;
  }
</style>