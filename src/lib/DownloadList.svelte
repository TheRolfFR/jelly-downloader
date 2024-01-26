<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'

  import { listen, type UnlistenFn } from '@tauri-apps/api/event'
  import { Heading, Secondary, GradientButton, Badge, P } from 'flowbite-svelte'
  import { FilePdfOutline, FileWordOutline, FileOutline, FileImageOutline, FileZipOutline, FileVideoOutline, FilePptOutline, FileLinesOutline, FileChartBarOutline, BookOutline, MobilePhoneSolid, FileMusicOutline } from 'flowbite-svelte-icons'
  import { DownloadOutline, CheckOutline, ClockOutline } from 'flowbite-svelte-icons'
  import { onDestroy, onMount } from 'svelte'
  import type { DownloadUpdate } from "$bindings/DownloadUpdate"
  import type { DownloadEvent } from '$bindings/DownloadEvent'


  onMount(() => {
    invoke("read_dir")
  })
  let download_map: Record<string, DownloadEvent> = {};
  let unlistener: UnlistenFn | undefined = undefined
  listen<DownloadUpdate>('DownloadEvent', ({ payload: received }) => {
    if(received.kind == 'error')
    {
      download_map[received.data.uuid].event = "error";
      console.error(received.data.msg);
      return;
    }

    const payload = received.data;
    download_map[payload.uuid] = payload;
  }).then(u => {
    unlistener = u
  })
  onDestroy(() => {
    if (unlistener !== undefined) unlistener()
  })

  const fileName = (path: string) => path.replaceAll('\\', '/').split('/').pop() as string
  const fileExtension = (filename: string) => filename.split('.').pop()?.toLowerCase()

  function fileIcon(path: string | null) {
    if(path == null) return FileOutline
    // Extract file extension
    const file_name = fileName(path)
    const file_extension = fileExtension(file_name)

    if (file_extension == undefined) return FileOutline

    // Map file extensions to Font Awesome icons
    const iconMap: Record<string, typeof FileOutline> = {
      txt: FileLinesOutline,
      ini: FileLinesOutline,
      css: FileLinesOutline,
      scss: FileLinesOutline,
      doc: FileWordOutline,
      docx: FileWordOutline,
      ppt: FilePptOutline,
      xlsx: FileChartBarOutline,
      pdf: FilePdfOutline,
      jpg: FileImageOutline,
      jpeg: FileImageOutline,
      png: FileImageOutline,
      svg: FileImageOutline,
      webp: FileImageOutline,
      gif: FileImageOutline,
      zip: FileZipOutline,
      jar: FileZipOutline,
      gz: FileZipOutline,
      mp4: FileVideoOutline,
      mp3: FileMusicOutline,
      epub: BookOutline,
      apk: MobilePhoneSolid,

      default: FileOutline,
    }

    return iconMap[file_extension] || iconMap['default']
  }

  function humanFileSize(size: number) {
    var i = size == 0 ? 0 : Math.floor(Math.log(size) / Math.log(1024))
    let res: any = (size / Math.pow(1024, i)) as number
    return (res.toFixed(2) * 1).toString() + ' ' + ['B', 'kB', 'MB', 'GB', 'TB'][i]
  }

  let sorted_downloads: DownloadEvent[] = []
  $: {
    sorted_downloads = Object.values(download_map)
    sorted_downloads.sort((a, b) => (b.filetime || 0) - (a.filetime || 0))
  }
</script>

<Heading tag="h5">Latest downloads</Heading>

<div id="download-list">
  {#if sorted_downloads.length > 0}
    {#each sorted_downloads as download}
      <div
        class="px-4 py-2 bg-neutral-600 rounded-lg cursor-pointer download-item flex flex-nowrap items-center gap-4 text-neutral-300"
      >
        <svelte:component this={fileIcon(download.path)} />

        <div class="flex-grow">
          {download.path ? fileName(download.path) : download.uuid}
          <div class="details flex items-center gap-1">
            <Badge class="bg-neutral-800">{humanFileSize(download.downloaded)}</Badge>
            {#if download.event == 'done'}
              <Badge color="green"><CheckOutline class="h-2.5 w-2.5 me-1" />Done</Badge>
            {:else if download.event == 'downloading'}
              <Badge color="blue"><DownloadOutline class="h-2.5 w-2.5 me-1" />In progress</Badge>
            {:else if download.event == 'pending' }
              <Badge color="blue"><ClockOutline class="h-2.5 w-2.5 me-1" />In progress</Badge>
            {:else}
              <Badge color="red">Error</Badge>
            {/if}
          </div>
        </div>

        {#if download.event == 'done'}
          <GradientButton outline color="purpleToBlue" class="!bg-transparent flex-shrink" on:click={() => invoke('open', { filePath: download.path })}>
              Open
          </GradientButton>
        {/if}
      </div>
    {/each}
  {:else}
    <p class="italic"><Secondary class="font-normal">Nothing to see here</Secondary></p>
  {/if}
</div>

<style lang="scss">
  $app_half_margin: 0.6rem;
  #download-list :global(> * + *) {
    margin-top: $app_half_margin;
  }

  .download-item {
    fonst-size: 14px;

    .flex-grow {
      font-size: 1rem;
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
      color: white;
      font-weight: 500;
    }

    .details {
      @apply text-gray-400;
      font-weight: normal;
    }
  }
</style>
