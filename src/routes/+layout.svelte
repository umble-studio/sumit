<script lang="ts">
    import "../app.css"
</script>

<main class="w-screen h-screen grid grid-rows-[58px_1fr_44px] ">
    <section class="flex border-b-[1px] border-white/[.1] justify-start items-center p-4 space-x-2">
        <h3 class="text-white">Header</h3>
    </section>
    <section class="">
        <slot></slot>
    </section>
    <section class="bg-black/[.35] flex justify-start items-center p-4 space-x-2">
        <h3 class="text-white">Footer</h3>
    </section>
</main>