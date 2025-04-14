<script lang="ts">
</script>

<div class="flex items-center justify-center aspect-square size-full">
	<span
		class="inline-block border-3 border-inherit border-full loader aspect-square size-full"
		style="border-bottom-color: transparent; border-radius: 50%; box-sizing: border-box; animation: rotation 1s linear infinite;"
	></span>

	<style>
		@keyframes rotation {
			0% {
				transform: rotate(0deg);
			}
			100% {
				transform: rotate(360deg);
			}
		}
	</style>
</div>
