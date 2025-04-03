<script lang="ts">
	import Loader from '../../components/Loader.svelte';
	import Background from '../../assets/background.jpg';
	import Config from '../../config';

	let loading = $state(false);
	let buttonState: 'login' | 'signup' = $state('login');

	function switchButtonState() {
		buttonState = buttonState === 'login' ? (buttonState = 'signup') : (buttonState = 'login');
	}

	async function login(event: Event) {
		event.preventDefault();

		if (loading) return;
		loading = true;

		const username = document.getElementById('Username') as HTMLInputElement;
		const password = document.getElementById('Password') as HTMLInputElement;

		try {
			const response = await fetch(`${Config.api}/login`, {
				method: 'POST',
				headers: {
					username: username.value,
					password: password.value
				}
			});

			const json = await response.json();
			console.log(json);
		} catch {}

		loading = false;
	}

	async function signup(event: Event) {
		event.preventDefault();

		if (loading) return;
		loading = true;

		const username = document.getElementById('Username') as HTMLInputElement;
		const password = document.getElementById('Password') as HTMLInputElement;

		try {
			const response = await fetch(`${Config.api}/signup`, {
				method: 'POST',
				headers: {
					username: username.value,
					password: password.value
				}
			});

			const json = await response.json();
			console.log(json);
		} catch {}

		loading = false;
	}
</script>

<div class="flex items-center justify-center inset-shadow-black-2x h-dvh w-dvw bg-gray-950">
	<form class="flex flex-col items-center justify-center gap-3 p-4 w-70 rounded-4xl bg-neutral-900">
		<p class="text-2xl font-bold text-left text-white">Login</p>
		{@render input('Username', 'text')}
		{@render input('Password', 'password')}
		<hr />
		{#if buttonState === 'login'}
			<button
				type="submit"
				onclick={login}
				class="w-full px-4 py-2 font-bold text-black duration-200 bg-blue-500 rounded-full shadow-xl cursor-pointer hover:bg-blue-600"
				>{@render loader('Login')}</button
			>

			<button
				onclick={switchButtonState}
				class="text-sm text-center text-blue-500 duration-100 cursor-pointer hover:text-blue-600"
				>Don't have an account?</button
			>
		{:else}
			<button
				type="submit"
				onclick={signup}
				class="w-full px-4 py-2 font-bold text-black duration-200 bg-blue-500 rounded-full shadow-xl cursor-pointer hover:bg-blue-600"
				>{@render loader('Signup')}</button
			>

			<button
				onclick={switchButtonState}
				class="text-sm text-center text-blue-500 duration-100 cursor-pointer hover:text-blue-600"
				>Already have an account?</button
			>
		{/if}
	</form>
</div>

{#snippet loader(text: String)}
	{#if loading}
		<div class="flex items-center justify-center h-6">
			<Loader />
		</div>
	{:else}
		{text}
	{/if}
{/snippet}

{#snippet input(title: string, type: 'text' | 'password' | 'email')}
	<div class="w-full">
		<p
			class="relative px-2 py-0 text-sm font-medium left-4 top-2 w-fit bg-neutral-900 text-neutral-500"
		>
			{title}
		</p>
		<input
			id={title}
			name={title}
			{type}
			class="z-10 w-full rounded-full px-4 py-1.5 text-base text-white outline-1 outline-neutral-500"
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
