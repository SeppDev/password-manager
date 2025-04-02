<script lang="ts">
	import Topbar from './Topbar.svelte';
	import Footer from './Footer.svelte';
	let { children } = $props();
</script>

<div class="bg-gray-200">
	<main class="min-h-svh">
		<Topbar></Topbar>
		{@render children()}
	</main>
	<Footer></Footer>
</div>
