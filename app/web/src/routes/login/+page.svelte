<script lang="ts">
	import Loader from '../../components/Loader.svelte';
	import Background from '../../assets/background.jpg';
	import Config from '../../config';

	let loading = $state(false);

	async function login(event: Event) {
		event.preventDefault();
		if (loading) return;
		loading = true;

		const username = document.getElementById('Username') as HTMLInputElement;
		const password = document.getElementById('Password') as HTMLInputElement;

		const response = await fetch(`${Config.api}/signup`, {
			method: 'POST',
			headers: {
				username: username.value,
				password: password.value
			}
		});

		loading = false;
	}
</script>

<div class="flex items-center justify-center inset-shadow-black-2x h-dvh w-dvw bg-neutral-800">
	<div class="flex flex-col items-center justify-center gap-2 py-4 w-70 rounded-2xl bg-neutral-100">
		<p class="text-2xl font-medium text-left">Login</p>
		{@render input('Username', 'text')}
		{@render input('Password', 'password')}
		<hr class="text-gray-800" />
	</div>

	<!-- <div
		style="position: fixed;top: 0;left: 0;width: 100%;height: 100%;box-shadow: 0 0 200px rgba(0,0,0,0.9) inset;"
	></div> -->
</div>

{#snippet input(title: string, type: 'text' | 'password' | 'email')}
	<div>
		<p class="relative px-2 py-0 text-base top-2.5 left-4.5 w-fit bg-neutral-100">{title}</p>
		<input
		id={title}
		name={title}
		{type}
		class="rounded-full z-10 px-4 py-1.5 outline-1 outline-gray-800"
		/>
	</div>
{/snippet}

<!-- <div class="bg-black h-dvh">
	<div class="flex items-center justify-center w-full text-white h-100 max-m-screen">
		<div class="bg-gray-900 w-80 rounded-2xl">
			<form class="flex flex-col w-full h-full gap-4 p-3">
				{@render input('Username', 'text')}
				{@render input('Password', 'password')}
				<hr class="text-gray-800" />
				<button
					type="submit"
					onclick={login}
					class="w-auto p-2 font-bold duration-200 bg-indigo-900 rounded-full cursor-pointer h-11 hover:bg-indigo-950"
				>
					{#if loading}
						<div class="flex items-center justify-center h-full">
							<Loader />
						</div>
					{:else}
						<p>Login</p>
					{/if}
				</button>
				<a
					href="/skibidi"
					class="text-center text-blue-500 duration-100 cursor-pointer hover:text-blue-700"
					>Don't have an account yet?</a
				>
			</form>
		</div>
	</div>
</div> -->
