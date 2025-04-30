<script lang="ts">
	import Thumbnail from '../assets/img/password.png';
	import Button from '../shared/Button.svelte';
	import Layout from '../components/Layout.svelte';
</script>

<Layout>
	<div class="flex h-140 w-full bg-white text-white">
		<div class="flex h-full grow-3 flex-col items-center justify-center gap-10">
			<div class="flex flex-col items-center">
				<p class="text-4xl font-medium">AuraPass</p>
				<p class="text-xl">Smart. Secure. Effortless.</p>
			</div>

			<div class="flex gap-3">
				<Button href="register" text="Register"></Button>
				<Button href="download" text="Download"></Button>
			</div>
		</div>
		<div class="flex aspect-3/2 h-full not-sm:hidden not-xl:hidden">
			<img src={Thumbnail} class="object-cover" alt="thumbnail" />
		</div>
	</div>
</Layout>
