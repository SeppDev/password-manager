<div class="flex items-center justify-center aspect-square size-full">
	<span class="loader"></span>

	<style>
		.loader {
			aspect-ratio: 1 / 1;
			height: 100%;
			border: 4px solid rgb(0,0,0);
			border-bottom-color: transparent;
			border-radius: 50%;
			display: inline-block;
			box-sizing: border-box;
			animation: rotation 1s linear infinite;
		}

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
