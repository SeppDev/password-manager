<script>
    import Thumbnail from "../assets/password.png"
	import Layout from '../components/Layout.svelte';
</script>

<Layout>
	<div class="flex w-full text-white bg-blue-600 h-140">
		<div class="flex flex-col items-center justify-center h-full gap-10 grow-3">
			<div class="flex flex-col items-center">
				<p class="text-4xl font-medium">AuraPass</p>
				<p class="text-xl">Smart. Secure. Effortless.</p>
			</div>

			<div class="flex gap-3">
				<button
					class="px-4 py-2 font-medium duration-200 bg-blue-800 rounded-full shadow-xl cursor-pointer hover:bg-blue-700"
					>Get started</button
				>
				<a href="./register"
					class="px-4 py-2 font-medium text-black duration-200 bg-blue-400 rounded-full shadow-xl cursor-pointer hover:bg-blue-500"
					>Register</a
				>
			</div>
		</div>
		<div class="flex h-full not-sm:hidden aspect-3/2 not-xl:hidden"> 	
			<img src={Thumbnail} class="object-cover" alt="thumbnail">
		</div>
	</div>
</Layout>
