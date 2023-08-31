<script lang="ts">
	export let contents: string;
	export let onClick: () => void;
</script>

<button on:click={onClick}>{contents}</button>

<style>
	button {
		font-family: 'FiraCode';
		color: #ebdbb2;
		background-color: black;
		border: 1px solid #ebdbb2;
		border-radius: 4px;
		width: 10em;
		height: 60%;
		margin-left: 5px;
	}

	button:hover {
		background-color: gray;
	}
</style>
