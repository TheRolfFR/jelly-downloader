<script lang="ts">
  import { BottomNav, BottomNavItem, Heading, P, Secondary } from 'flowbite-svelte'
  import { HomeSolid, DownloadSolid, AdjustmentsVerticalOutline } from 'flowbite-svelte-icons'

  import DownloadForm from './lib/DownloadForm.svelte'
  import DownloadList from './lib/DownloadList.svelte'

  let activeUrl = '#'
</script>

<div id="main-content">
  <div>
    <Heading tag="h4">FileDownloader</Heading>
    <Heading tag="h5"><Secondary>by TheRolf</Secondary></Heading>
  </div>
  <div id="techstack">
    <Heading tag="h5">Made with</Heading>
    <div id="logos">
      <div><img src="/vite.svg" alt="vite" /></div>
      <div><img src="/tauri.svg" alt="tauri" /></div>
      <div><img src="/svelte.svg" alt="svelte" /></div>
      <div><img src="/flowbite.svg" alt="flowbite" /></div>
    </div>
    <div id="findnames">
      <div>vite</div>
      <div>tauri</div>
      <div>svelte</div>
      <div>flowbite</div>
    </div>
  </div>

  <DownloadForm />
  <DownloadList />
</div>

<BottomNav id="nav" {activeUrl} position="static" outerClass="!border-none w-full z-50 bg-gradient-to-br dark:bg-gradient-to-br from-purple-600 to-blue-500" classInner="grid-cols-3 bg-transparent text-white">
  <BottomNavItem btnName="Home" href="#" class="item">
    <HomeSolid class="w-5 h-5 mb-1" />
  </BottomNavItem>
  <BottomNavItem btnName="Downloads" href="#Downloads" class="item">
    <DownloadSolid class="w-5 h-5 mb-1" />
  </BottomNavItem>
  <BottomNavItem btnName="Settings" href="#Settings" class="item">
    <AdjustmentsVerticalOutline class="w-5 h-5 mb-1" />
  </BottomNavItem>
</BottomNav>

<style lang="scss">
  @use 'sass:math';

  :global(#nav a, #nav a span) {
    color: inherit !important;
  }

  $app_margin: 1.2rem;
  #main-content {
    @apply bg-gray-900;
    @apply dark:bg-neutral-900;
    color: white;
    padding: $app_margin;
    height: calc(100vh - 4rem);
    overflow: hidden auto;

    & :global(> * + *) {
      margin-top: $app_margin !important;
    }
  }

  :global(#open) {
    width: 100%;
    max-width: none;
    display: block;
    padding: 12px;
    text-align: center;
  }

  $drop_shadow_size: 0.5rem;
  #logos {
    padding: $drop_shadow_size 0;

    &,
    & + #findnames {
      display: grid;
      grid-auto-flow: column;
      grid-auto-columns: 1fr;
      gap: $app_margin;
    }
    & + #findnames {
      text-align: center;
      text-transform: capitalize;
      margin-top: math.div($drop-shadow-size, 2);
    }

    & > div {
      display: flex;
      align-items: center;

      & > img {
        flex-grow: 1;
        filter: drop-shadow(0 0.3rem 4px rgba(0, 0, 0, 0.4));
      }
    }
  }

  :global(#nav) {
    @apply drop-shadow-[0_5px_5px_rgba(0,0,0,0.7)];
  }
</style>
