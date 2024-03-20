<script lang="ts">
    import Icon, { type IconifyIcon } from "@iconify/svelte";
    import type { Gradient } from "./Utility";

    export let icon: string | IconifyIcon = "";
    export let color: string | undefined = "white";
    export let gradient: Gradient | undefined = {
        from: "from-blue-400",
        to: "to-blue-600",
        orientation: "bg-gradient-to-br",
    };
</script>

<main class="flex justify-center items-center w-6 h-6 rounded-lg {gradient?.orientation || "bg-gradient-to-br"} {gradient?.from} {gradient?.to}">
    <Icon class="w-full h-full p-1" {icon} {color} />
</main>