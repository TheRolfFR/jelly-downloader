<script lang="ts">
  import { Heading, Input, ButtonGroup, Button, GradientButton } from "flowbite-svelte"
  import { slide } from "svelte/transition"
  import { CloseOutline } from "flowbite-svelte-icons"
  import { DownloadSolid } from "flowbite-svelte-icons"
  import { invoke } from "@tauri-apps/api/core"

  let value = "https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf"
  let clear = () => { value = "" }

  let download = () => invoke("download", { url: value })
</script>

<Heading tag="h5">Download file</Heading>

<div id="download-form">
    <ButtonGroup class="w-full">
        <Input bind:value id="large-input" placeholder="Enter URL here" class="!bg-neutral-600 dark:!bg-neutral-600 border-none " />
        {#if value }
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <div transition:slide={{ axis: 'x' }} on:click={clear}>
                <Button id="clear-button" class="!rounded-none !rounded-r-lg h-full dark:bg-neutral-600 border-none hover:dark:bg-neutral-600">
                    <CloseOutline class="w-4 h-4 text-white" />
                </Button>
            </div>
        {/if}
    </ButtonGroup>
    <GradientButton disabled={!value} color="purpleToBlue" class="w-full" on:click={download}>
        <DownloadSolid class="w-4 h-4 me-2" /> Download
    </GradientButton>
</div>

<style lang="scss">
    $app_half_margin: 0.6rem;
    #download-form :global( > * + * ) {
        margin-top: $app_half_margin;
    }
    :global(#clear-button) {
        width: 40px;
    }
</style>
