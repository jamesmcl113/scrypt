<script lang="ts">
	import { SvelteToast } from '@zerodevx/svelte-toast';

	const options = {
		pausable: true
	};
</script>

<div class="notify-wrapper">
	<SvelteToast {options} />
</div>

<style>
	.notify-wrapper {
		font-size: 0.75rem;
	}

	:global(._toastMsg) {
		overflow: hidden;
	}
</style>
