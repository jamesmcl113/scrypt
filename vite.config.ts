import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		port: 5173
	},
	optimizeDeps: {
		exclude: ['@codemirror', '@codemirror/lang-markdown', 'cm6-theme-gruvbox-dark']
	}
});
