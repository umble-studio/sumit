<script lang="ts">
    import "../app.css"
    import TabIcon from "../components/TabIcon.svelte";
</script>

<main class="w-screen h-screen grid grid-rows-[60px_1fr_44px] ">
    <section class="flex border-b-[1px] border-white/[.1] justify-start items-center p-4 space-x-2">
        <h3 class="text-white">Header</h3>
    </section>
    <section class="">
        <slot></slot>
    </section>
    <section class="bg-black/[.35] flex justify-start items-center p-3 space-x-2">
        <div class="flex items-center space-x-2">
            <TabIcon icon="ic:baseline-assistant" gradient={{from: "blue-400", to: "blue-600"}} />
            <h3 class="text-white">Footer</h3>
        </div>
    </section>
</main>