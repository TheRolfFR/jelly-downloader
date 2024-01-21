<script lang="ts">
  import 'carbon-components-svelte/css/all.css'

  import { Header, HeaderNav, HeaderNavItem, HeaderNavMenu, SideNav, SideNavItems, SideNavMenu, SideNavMenuItem, SideNavLink, SideNavDivider, SkipToContent, Content, Grid, Row, Column, Link, ImageLoader } from 'carbon-components-svelte'

  let isSideNavOpen = false

  let theme = 'g90' // "white" | "g10" | "g80" | "g90" | "g100"

  $: document.documentElement.setAttribute('theme', theme)

  import Greet from './lib/Greet.svelte'
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

<SideNav bind:isOpen={isSideNavOpen}>
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
  <Grid fullWidth noGutter class="grid">
    <Row id="logos">
      <Column><ImageLoader style="filter: drop-shadow(0 0 0.7em #747bff)" src="/vite.svg" /></Column>
      <Column><ImageLoader style="filter: drop-shadow(0 0 0.7em #24c8db)" src="/tauri.svg" /></Column>
      <Column><ImageLoader style="filter: drop-shadow(0 0 0.7em #ff3e00)" src="/svelte.svg" /></Column>
      <Column><ImageLoader style="filter: drop-shadow(0 0 0.7em #42AAEA)" src="/carbon.png" /></Column>
    </Row>
    <Row>
      <Column><p>Click on the Tauri, Vite, and Svelte logos to learn more.</p></Column>
    </Row>
    <Row>
      <Column><Greet /></Column>
    </Row>
  </Grid>
</Content>

<style lang="scss">
  :global(#main-content) {
    align-items: center;
    text-align: center;
    padding: 1.2rem;

    &, & :global(.grid) {
      display: flex;
      flex-direction: column;
      gap: 1.2rem;
    }

    :global(#logos) {
      align-items: center;
    }
  }
</style>
