<script lang="ts">
	import { onMount } from 'svelte';
	import Loader from '../../components/Loader.svelte';
	import Config from '../../config';

	type Response = {
		message?: String;
		token?: String;
	};

	let errorMessage: String | undefined = $state(undefined);
	let loading = $state(false);
	let buttonState: 'login' | 'signup' = $state('login');
	let page: 'loading' | 'loggedin' | 'register' = $state('loading');

	function switchButtonState() {
		buttonState = buttonState === 'login' ? (buttonState = 'signup') : (buttonState = 'login');
	}

	async function handleResponse(response: Response) {
		errorMessage = response.message;
		if (!response.token) return

		console.log(response);
		page = 'loggedin';
		document.cookie = `token=${response.token};`
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

			const json = (await response.json()) as Response;
			await handleResponse(json);
		} catch {
			errorMessage = 'Something went wrong';
		}

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

			const json = (await response.json()) as Response;
			await handleResponse(json);
		} catch {
			errorMessage = 'Something went wrong';
		}

		loading = false;
	}


	onMount(async () => {
		const cookie = document.cookie;
		if (cookie.length === 0) {
			page = 'register';
			return
		}
		
		const response = await fetch(`${Config.api}/userinfo`);
		const json = await response.json();
		
		if (!json) {
			page = "register"
			return
		}

	});
</script>

<div
	class="flex flex-col items-center justify-center gap-4 inset-shadow-black-2x h-dvh w-dvw bg-gray-950"
>
	{#if page === 'register'}
		<form
			class="flex flex-col items-center justify-center gap-3 p-4 w-70 rounded-4xl bg-neutral-900"
		>
			<p class="text-2xl font-bold text-left text-white">Login</p>
			{#if errorMessage !== undefined}
				<div class="w-full text-black bg-red-500 rounded-2xl outline-2 outline-red-600">
					<p class="px-4 py-2">{errorMessage}</p>
				</div>
			{/if}

			{@render input('Username', 'text')}
			{@render input('Password', 'password')}
			<hr />
			{#if buttonState === 'login'}
				{@render loaderButton('Login', login, "Don't have an account?")}
			{:else}
				{@render loaderButton('Sign up', signup, 'Already have an account?')}
			{/if}
		</form>
	{:else if page === 'loading'}
		<div class="w-20 h-20 aspect-square">
			<Loader />
		</div>
	{:else if page === 'loggedin'}
		<p class="text-3xl text-white">Logged in</p>
	{:else}
		<p class="text-3xl font-black text-white">Oops!</p>
		<p class="text-white">Something went wrong</p>
	{/if}
</div>

{#snippet loaderButton(text: String, onclick: (event: Event) => Promise<void>, other: String)}
	<button
		type="submit"
		{onclick}
		class="flex items-center justify-center w-full h-10 p-2 font-bold text-black transition duration-200 bg-blue-500 rounded-full shadow-xl cursor-pointer hover:bg-blue-600"
	>
		{#if loading}
			<span class="left-0 flex items-center justify-center h-full aspect-square">
				<Loader color="rgb(0, 0, 0)" />
			</span>
		{:else}
			<span class="text-center size-full">
				{text}
			</span>
		{/if}
	</button>
	<button
		onclick={switchButtonState}
		class="text-sm text-center text-blue-500 duration-100 cursor-pointer hover:text-blue-600"
		>{other}</button
	>
{/snippet}

{#snippet input(title: string, type: 'text' | 'password' | 'email')}
	<div class="w-full">
		<p class="relative px-2 py-0 text-sm left-4 top-2 w-fit bg-neutral-900 text-neutral-500">
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
