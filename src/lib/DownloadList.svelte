<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'

  import { listen, type UnlistenFn } from '@tauri-apps/api/event'
  import { Heading, Secondary, GradientButton, Badge } from 'flowbite-svelte'
  import { FilePdfOutline, FileWordOutline, FileOutline, FileImageOutline, FileZipOutline, DownloadOutline, CheckOutline, FileVideoOutline } from 'flowbite-svelte-icons'
  import { onDestroy } from 'svelte'

  interface DownloadItem {
    path: string
    downloaded: number
    status: string
    id: number
  }
  let id = 0
  let download_map: Record<string, DownloadItem> = {}
  download_map['a'] = {
    path: '/storage/emulated/0/Android/data/com.therolf.jelly_downloader/files/Download/BigBuckBunny.mp4',
    downloaded: 160000000, // 158.2MB
    status: 'Done',
    id: id++
  }
  download_map['b'] = {
    path: '/storage/emulated/0/Android/data/com.therolf.jelly_downloader/files/Download/dummy.pdf',
    downloaded: 13 * 1024 + 260,
    status: 'Done',
    id: id++
  }
  download_map['c'] = {
    path: '/storage/emulated/0/Android/data/com.therolf.jelly_downloader/files/Download/the_rolf_no_bg.png',
    downloaded: 2 * 1024 + 40,
    status: 'Done',
    id: id++
  }
  download_map['d'] = {
    path: '/storage/emulated/0/Android/data/com.therolf.jelly_downloader/files/Download/v2.11.1.zip',
    downloaded: 4 * 1024 * 1024 + 11 * 1024,
    status: 'Done',
    id: id++
  }

  interface DownloadEvent {
    uuid: string
    path: string
    event: string
  }
  let unlistener: UnlistenFn | undefined = undefined
  listen<DownloadEvent>('DownloadEvent', ({ payload }) => {
    if (!(payload.uuid in download_map)) {
      download_map[payload.uuid] = {
        path: '',
        downloaded: 0,
        status: '',
        id: id++
      }
    }
    download_map[payload.uuid].status = payload.event
    if (payload.path) download_map[payload.uuid].path = payload.path
  }).then(u => {
    unlistener = u
  })
  onDestroy(() => {
    if (unlistener !== undefined) unlistener()
  })

  const fileName = (path: string) => path.split('/').pop() as string
  const fileExtension = (filename: string) => filename.split('.').pop()?.toLowerCase()

  function fileIcon(file_name: string) {
    // Extract file extension
    const file_extension = fileExtension(file_name)

    if (file_extension == undefined) return FileOutline

    // Map file extensions to Font Awesome icons
    const iconMap: Record<string, typeof FileOutline> = {
      pdf: FilePdfOutline,
      doc: FileWordOutline,
      docx: FileWordOutline,
      txt: FileOutline,
      jpg: FileImageOutline,
      jpeg: FileImageOutline,
      png: FileImageOutline,
      zip: FileZipOutline,
      gz: FileZipOutline,
      mp4: FileVideoOutline,
      default: FileOutline
    }

    return iconMap[file_extension] || iconMap['default']
  }

  function humanFileSize(size: number) {
    var i = size == 0 ? 0 : Math.floor(Math.log(size) / Math.log(1024))
    let res: any = (size / Math.pow(1024, i)) as number
    return (res.toFixed(2) * 1).toString() + ' ' + ['B', 'kB', 'MB', 'GB', 'TB'][i]
  }

  let sorted_downloads: DownloadItem[] = []
  $: {
    sorted_downloads = Object.values(download_map)
    sorted_downloads.sort((a, b) => b.id - a.id)
  }
</script>

<Heading tag="h5">Latest downloads</Heading>

<div id="download-list">
  {#if sorted_downloads.length > 0}
    {#each sorted_downloads as download}
      <div
        class="px-4 py-2 bg-neutral-600 rounded-lg cursor-pointer download-item flex flex-nowrap items-center gap-4 text-neutral-300"
      >
        <svelte:component this={fileIcon(fileName(download.path))} />

        <div class="flex-grow">
          {fileName(download.path)}
          <div class="details flex items-center gap-1">
            <Badge class="bg-neutral-800">{humanFileSize(download.downloaded)}</Badge>
            {#if download.status == 'Done'}
              <Badge color="green"><CheckOutline class="h-2.5 w-2.5 me-1" />Done</Badge>
            {:else if download.status == 'Progress'}
              <Badge color="blue"><DownloadOutline class="h-2.5 w-2.5 me-1" />In progress</Badge>
            {:else}
              <Badge color="red">Error</Badge>
            {/if}
          </div>
        </div>

        {#if download.status == 'Done'}
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
