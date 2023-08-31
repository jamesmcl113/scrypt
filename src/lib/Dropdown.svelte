<script lang="ts" context="module">
	export interface DropdownOption {
		name: string;
		cb: () => void;
	}
</script>

<script lang="ts">
	import Button from './Button.svelte';

	export let options: DropdownOption[];

	let showMenu = false;

	const toggle = () => {
		showMenu = !showMenu;
	};

	const onLeave = () => {
		showMenu = false;
	};
</script>

<div class="dropdown" on:mouseleave={onLeave} role="button" tabindex="0">
	<Button contents={'Menu'} onClick={toggle} />
	{#if showMenu}
		<div class="dropdown-contents">
			{#each options as option}
				<Button onClick={option.cb} contents={option.name} />
			{/each}
		</div>
	{/if}
</div>

<style>
	.dropdown {
		position: relative;
		display: flex;
		align-items: center;
		height: 100%;
	}

	.dropdown-contents {
		position: absolute;
		top: 42px;
		color: #ebdbb2;
		width: 100%;
		z-index: 2;
	}
</style>
