<script lang="ts">
	import CodeMirror from 'svelte-codemirror-editor';
	import { closeBrackets } from '@codemirror/autocomplete';
	import { markdown } from '@codemirror/lang-markdown';
	import { gruvboxDark } from 'cm6-theme-gruvbox-dark';

	import { editorContent } from './stores/editorContent';

	const customSetup = [closeBrackets()];
</script>

<div>
	<CodeMirror
		bind:value={$editorContent}
		basic={false}
		lang={markdown()}
		theme={gruvboxDark}
		extensions={customSetup}
		styles={{
			'&': {
				height: '100%',
				'min-width': '0'
			},
			'.cm-scroller': {
				overflow: 'auto'
			}
		}}
	/>
</div>

<style>
	div {
		display: flex;
		width: 50%;
		height: 100%;
	}

	:global(.codemirror-wrapper) {
		width: 100%;
	}

	div::-webkit-scrollbar {
		background-color: #282828;
	}

	div::-webkit-scrollbar-thumb {
		background-color: #484848;
		border-radius: 5px;
	}
</style>
