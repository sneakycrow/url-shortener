<script lang="ts">
	import { enhance } from '$app/forms';

	/** @type {import('./$types').ActionData} */
	export let form;
</script>

<main class="h-screen w-screen flex items-center justify-center">
	<img src="/logo.svg" alt="logo of sneaky crow, creator of app" height={200} width={200} />
	<form
		method="POST"
		use:enhance
		class="flex flex-col items-start justify-center w-full max-w-[600px] mx-10 my-4 space-y-4"
	>
		<label for="link" class="text-left">shorten a link</label>
		<input
			class="placeholder:text-gray-300 w-full border py-2 px-4 rounded-md shadow-md"
			type="text"
			name="link"
			placeholder="paste link here"
		/>
		<button
			type="submit"
			class="uppercase text-sm text-gray-400 font-semibold shadow-md rounded-md border-2 border-gray-300 px-4 py-2 max-w-[200px]"
			>shorten</button
		>
		{#if form?.link}
			<p>link: {form.link}</p>
		{/if}
	</form>
</main>
