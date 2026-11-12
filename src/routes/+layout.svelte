<script lang="ts">
    import "../app.css";

    let { children } = $props();

    let windowWidth = $state(800);
    let windowHeight = $state(600);
</script>

<svelte:window bind:innerWidth={windowWidth} bind:innerHeight={windowHeight}/>

<div
    style:width="{windowWidth}px"
    style:height="{windowHeight}px"
>
    <div class="h-full w-full flex flex-col">
        <div class="grow flex">
            <div class="shrink-0 h-full">
                Sidebar
            </div>
            <div>
                {@render children()}
            </div>
        </div>

        <div
            class="grow-0 shrink-0 w-full text-sm leading-[1em] border-t border-t-secondary px-2 py-2 text-secondary"
        >
            This is status bar
        </div>
    </div>
</div>
