<script lang="ts">
	import type { Color } from '../types/colors';

	let { color }: { color?: Color } = $props();
	if (color === undefined) {
		color = 'rgb(255, 255, 255)';
	}
</script>

<div class="flex items-center justify-center aspect-square size-full">
	<span
		class="inline-block size-full loader aspect-square"
		style="border: 4px solid {color}; 			
    border: 5px solid {color};
    border-bottom-color: transparent;
    border-radius: 50%;
    box-sizing: border-box;
    animation: rotation 1s linear infinite;"
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
