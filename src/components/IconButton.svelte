<script lang="ts">
    import Icon, { type IconifyIcon } from "@iconify/svelte";
    import type { Gradient } from "./Utility";

    export let icon: string | IconifyIcon = "";
    export let color: string | undefined = "white";
    export let background: string | undefined = "bg-blue-100/[.1]";
</script>

<main class="flex justify-center items-center w-6 h-6 rounded-lg {background} hover:bg-blue-100/[.2] transition-colors duration-150">
    <Icon class="w-full h-full p-1" {icon} {color} />
</main>