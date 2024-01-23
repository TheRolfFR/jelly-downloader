<script lang="ts">
  import { listen, type UnlistenFn } from '@tauri-apps/api/event'
  import 'carbon-components-svelte/css/all.css'
  import Greet from './lib/Greet.svelte'
  import { Header, HeaderNav, HeaderNavItem, HeaderNavMenu, SideNav, SideNavItems, SideNavMenu, SideNavMenuItem, SideNavLink, SideNavDivider, SkipToContent, Content, Grid, Row, Column, Link, ImageLoader, Button } from 'carbon-components-svelte'
  import { onDestroy } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'

  let isSideNavOpen = false

  let theme = 'g90' // "white" | "g10" | "g80" | "g90" | "g100"

  $: document.documentElement.setAttribute('theme', theme)

  let unlistener: UnlistenFn | undefined = undefined;
  
  listen("DownloadEvent", ({ payload }) => {
    console.log(payload);
  })
  .then(u => {
    unlistener = u
  })

  onDestroy(() => {
    if(unlistener !== undefined)
      unlistener()
  })

  function openDialog() {
    invoke("dialog")
  }
</script>

<Header  bind:isSideNavOpen>
  <svelte:fragment slot="platform">
    <div>Tauri with Carbon Svelte</div>
  </svelte:fragment>
  <svelte:fragment slot="skip-to-content">
    <SkipToContent />
  </svelte:fragment>
  <HeaderNav>
    <HeaderNavItem href="/" text="Link 1" />
    <HeaderNavItem href="/" text="Link 2" />
    <HeaderNavItem href="/" text="Link 3" />
    <HeaderNavMenu text="Menu">
      <HeaderNavItem href="/" text="Link 1" />
      <HeaderNavItem href="/" text="Link 2" />
      <HeaderNavItem href="/" text="Link 3" />
    </HeaderNavMenu>
    <HeaderNavItem href="/" text="Link 4" />
  </HeaderNav>
</Header>

<SideNav id="sidenav" bind:isOpen={isSideNavOpen}>
  <SideNavItems>
    <SideNavLink text="Link 1" />
    <SideNavLink text="Link 2" />
    <SideNavLink text="Link 3" />
    <SideNavMenu text="Menu">
      <SideNavMenuItem href="/" text="Link 1" />
      <SideNavMenuItem href="/" text="Link 2" />
      <SideNavMenuItem href="/" text="Link 3" />
    </SideNavMenu>
    <SideNavDivider />
    <SideNavLink text="Link 4" />
  </SideNavItems>
</SideNav>

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
