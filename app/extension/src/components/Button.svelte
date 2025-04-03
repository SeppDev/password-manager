<script lang="ts">
    let { onclick, children } = $props();
</script>

<button {onclick} class="px-4 py-2 font-bold text-black duration-200 bg-blue-500 rounded-full shadow-xl cursor-pointer hover:bg-blue-600">
    {@render children()}
</button>
