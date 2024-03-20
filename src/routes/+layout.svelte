<script lang="ts">
    import "../app.css"
    import SearchBar from "../components/SearchBar.svelte";
    import TabIcon from "../components/TabIcon.svelte";
    import IconButton from "../components/IconButton.svelte";
</script>

<main class="w-screen h-screen grid grid-rows-[60px_1fr_44px] border-x-[2px] border-t-[1px] border-b-[3px] border-white/[.2] rounded-[8px]">
    <section class="flex border-b-[1px] border-white/[.1] justify-start items-center p-4 space-x-2">
        <IconButton icon="ic:outline-keyboard-arrow-left" background="bg-blue-100/[.1]"/>
        <SearchBar placeholder="Search Store for Extensions"></SearchBar>
    </section>
    <section class="">
        <slot></slot>
    </section>
    <section class="bg-black/[.4] flex justify-start items-center p-3 space-x-2">
        <div class="flex items-center space-x-2">
            <TabIcon icon="ic:baseline-assistant" gradient={{from: "from-blue-300", to: "to-blue-500"}} />
            <h3 class="text-white">Footer</h3>
        </div>
    </section>
</main>