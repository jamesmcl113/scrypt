<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { editorContent } from './stores/editorContent';
	import hljs from 'highlight.js';

	let contents: string = '';

	editorContent.subscribe(async (val) => {
		const newContents: string = await invoke('md_to_html', { content: val });
		contents = newContents;
	});
</script>

<div class="contents">
	{@html contents}
</div>

<style>
	.contents {
		display: flex;
		flex-direction: column;
		width: 50%;
		margin-left: 1em;
		color: #ebdbb2;
		background-color: #403e3e;
		overflow: auto;

		font-size: 0.8em;
		font-family: 'FiraCode';
	}

	.contents > :global(p) {
		margin-top: 1.2em;
		margin-bottom: 1.2em;
	}

	.contents > :global(h1) {
		margin: 2px;
		font-weight: bold;
	}

	.contents > :global(h2) {
		margin: 2px;
	}

	.contents > :global(h3) {
		margin: 2px;
	}

	.contents > :global(ul) {
		margin-top: 0.5em;
		line-height: 10px;
	}

	.contents > :global(a:link) {
		color: red;
	}

	:global(code) {
		font-size: 1.2em;
		color: #f2e6c7;
		background-color: rgba(0, 0, 0, 0.2);
		border-radius: 3px;
		padding: 2px 4px;
	}

	:global(pre) > :global(code) {
		display: block;
		overflow-x: auto;
		margin-right: 1em;
		white-space: pre;
	}
</style>
