<script lang="ts">
	import CodeEditor from '$lib/CodeEditor.svelte';
	import Header from '$lib/Header.svelte';
	import Preview from '$lib/Preview.svelte';
	import Notifier from '$lib/Notifier.svelte';
	import { toast } from '@zerodevx/svelte-toast';

	import { onMount } from 'svelte';

	import { File } from '$lib/scripts/file';
	import { editorContent } from '$lib/stores/editorContent';

	let file: File | undefined;

	const onNewFile = async () => {
		try {
			let newFile = await File.init_empty();

			file = newFile;

			editorContent.set(newFile.buffer);
			toast.push('Opened file ' + newFile.path + '.');
		} catch (error) {
			console.log(error);
		}
	};

	const onOpenFile = async () => {
		try {
			// save currently opened file
			if (file) {
				file.save();
			}

			let newFile = await File.init();

			file = newFile;
			editorContent.set(newFile.buffer);
			toast.push('Opened file ' + newFile.path + '.');
		} catch (error) {
			console.log("Couldn't open file: " + error);
		}
	};

	editorContent.subscribe((newVal) => {
		if (file) {
			file.buffer = newVal;
		}
	});

	const onSaveFile = async () => {
		if (file) {
			await file.save();
			toast.push('Saved file successfully!');
		}
	};
</script>

<div class="main-window">
	<Notifier />
	<Header {onSaveFile} {onOpenFile} {onNewFile} />
	<div class="editor-pane">
		{#if file}
			<CodeEditor />
			<Preview />
		{:else}
			<p>Hit 'Open' to get started.</p>
		{/if}
	</div>
</div>

<style>
	:global(body) {
		margin: 0px;
		overflow: hidden;
		height: 100vh;
	}

	.main-window {
		display: flex;
		height: 100%;
		flex-direction: column;
		font-family: 'FiraCode';
	}

	.editor-pane {
		display: flex;
		height: 100%;
		white-space: pre-line;
		background-color: #403e3e;
		overflow: hidden;
	}
	.editor-pane {
		display: flex;
		height: 100%;
		white-space: pre-line;
		background-color: #403e3e;
		overflow: hidden;
	}
</style>
