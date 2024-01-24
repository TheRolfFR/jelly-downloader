<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { onDestroy } from 'svelte';

  import { TextInput } from "carbon-components-svelte";
  import { Button } from "carbon-components-svelte";
  // @ts-ignore
  import Download from "carbon-icons-svelte/lib/Download.svelte";

  let url = "";

  let download_list: Record<string, {
    path: string,
    downloaded: number,
    status: string
  }> = {}
  async function download(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const uuid = await invoke("download", { url }) as string
    download_list[uuid] = {
      path: "",
      downloaded: 0,
      status: ""
    }
  }

  interface DownloadEvent {
    uuid: string,
    path: string,
    event: string
  }
  let unlistener: UnlistenFn | undefined = undefined;
  listen<DownloadEvent>("DownloadEvent", ({ payload }) => {
    download_list[payload.uuid].status = payload.event
    if(payload.path)
      download_list[payload.uuid].path = payload.path
  })
  .then(u => {
    unlistener = u
  })
  onDestroy(() => {
    if(unlistener !== undefined)
      unlistener()
  })
</script>

<div>
  <form on:submit|preventDefault={download}>
    <div id="url-input">
      <TextInput placeholder="Enter URL" bind:value={url} />
    </div>
    <Button size="field" type="submit" kind="primary" icon={Download} iconDescription="Download" />
  </form>

  <div id="download-list">
    {#each Object.entries(download_list) as [uuid, download] }
      <div class="download-item">
        <div class="uuid">{ uuid }</div>
        <div class="path">{ download.path }</div>
        <div class="progress">{ download.downloaded }</div>
        <div class="status">{ download.status }</div>
        {#if download.status.toLowerCase() == "done" }
          <button class="open" on:click={() => invoke("open", { filePath: download.path }) }>Open</button>
        {/if}
      </div>
    {/each}
  </div>
</div>

<style lang="scss">
  form {
    display: flex;
    gap: 0.6rem;

    #url-input {
      flex-grow: 1;
    }
  }
  .download-item {
    border-bottom: 1px solid #ccc;
    font-size: 14px;
  }
</style>