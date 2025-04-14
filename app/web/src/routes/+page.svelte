<script lang="ts">
	import Thumbnail from '../assets/password.png';
	import Button from '../shared/Button.svelte';
	import Layout from '../components/Layout.svelte';
</script>

<Layout>
	<div class="flex w-full text-white bg-white h-140">
		<div class="flex flex-col items-center justify-center h-full gap-10 grow-3">
			<div class="flex flex-col items-center">
				<p class="text-4xl font-medium">AuraPass</p>
				<p class="text-xl">Smart. Secure. Effortless.</p>
			</div>

			<div class="flex gap-3">
				<Button href="/register" text="Register"></Button>
			</div>
		</div>
		<div class="flex h-full not-sm:hidden aspect-3/2 not-xl:hidden">
			<img src={Thumbnail} class="object-cover" alt="thumbnail" />
		</div>
	</div>
</Layout>

