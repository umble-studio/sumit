<script lang="ts">
    let search = "";
    export let placeholder: string | null | undefined = "Search";
</script>

<main class="group px-2 py-1 transition-colors duration-150">
    <input class="w-full bg-transparent text-white font-semibold caret-yellow-500 placeholder-blue-100/[.4]" bind:value={search} type="text" {placeholder}>
</main>