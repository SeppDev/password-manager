<script lang="ts">
	const { text, href }: { text: string; href: string } = $props();
</script>

<a class="underline" href={href}>{text}</a>
