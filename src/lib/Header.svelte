<script lang="ts">
	import Button from './Button.svelte';
	import Dropdown, { type DropdownOption } from './Dropdown.svelte';

	export let onSaveFile: () => void;
	export let onOpenFile: () => void;
	export let onNewFile: () => void;

	const options: DropdownOption[] = [
		{ name: 'Save', cb: onSaveFile },
		{ name: 'Open', cb: onOpenFile },
		{ name: 'New', cb: onNewFile }
	];
</script>

<div>
	<div class="file-buttons">
		<Dropdown {options} />
	</div>
	<h2>Scrypt</h2>
</div>

<style>
	div {
		font-family: 'FiraCode';
		height: 50px;
		color: white;
		background-color: black;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	h2 {
		margin: 5px;
		text-align: center;
	}

	.file-buttons {
		height: 100%;
		margin-left: 1em;
	}
</style>
