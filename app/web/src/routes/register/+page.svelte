<script lang="ts">
	import { onMount } from 'svelte';
	import Loader from '../../shared/Loader.svelte';
	import Config from '../../config';
	import Button from '../../shared/Button.svelte';

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
		if (!response.token) return;
		page = 'loggedin';
		document.cookie = `token=${response.token};`;
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

	function signout() {
		document.cookie = 'token=;expires=Thu, 01 Jan 1970 00:00:01 GMT';
		page = 'register';
	}

	onMount(async () => {
		const cookie = document.cookie;
		if (cookie.length === 0) {
			page = 'register';
			return;
		}

		try {
			const response = await fetch(`${Config.api}/authenticated`, {
				credentials: 'include'
			});
			if (response.status === 200) {
				page = 'loggedin';
				return;
			}
		} catch {}
		page = 'register';
	});
</script>

<div
	class="flex flex-col items-center justify-center gap-10 h-dvh w-dvw"
>
	{#if page === 'register'}
		<form
			class="flex flex-col items-center justify-center gap-3 p-6 shadow-xl not-dark:bg-white dark:bg-neutral-900 w-100 not-sm:rounded-none not-sm:size-full dark:shadow-black rounded-xl"
		>
			<p class="w-full text-2xl font-bold text-left">Login</p>
			{#if errorMessage !== undefined}
				<div class="w-full bg-red-200 rounded-lg not-dark:text-red-800 dark:text-red-100 dark:bg-red-500 not-dark:outline-1 not-dark:outline-red-400">
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
		<div class="w-20 aspect-square">
			<Loader />
		</div>
	{:else if page === 'loggedin'}
		<p class="text-4xl">Logged in</p>
		<div>
			<Button onclick={signout} text="Signout"></Button>
		</div>
	{:else}
		<p class="text-3xl font-black">Oops!</p>
		<p>Something went wrong</p>
	{/if}
</div>

{#snippet loaderButton(text: string, onclick: (event: Event) => Promise<void>, other: String)}
	<Button type="submit" {onclick} {text} />
	<button
		onclick={switchButtonState}
		class="text-sm text-center text-blue-500 duration-100 cursor-pointer hover:text-blue-600"
		>{other}</button
	>
{/snippet}

{#snippet input(title: string, type: 'text' | 'password' | 'email')}
	<div class="w-full">
		<p class="relative px-2 py-0 text-sm not-dark:bg-white dark:bg-neutral-900 left-4 top-2 w-fit text-neutral-400">
			{title}
		</p>
		<input
			id={title}
			{type}
			class="z-10 w-full px-4 py-3 text-base duration-100 rounded-lg dark:text-white not-dark:black outline-1 outline-neutral-400 focus:outline-blue-500 ring-blue-400 focus:ring-2"
		/>
	</div>
{/snippet}
