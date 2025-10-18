import { paraglideVitePlugin } from '@inlang/paraglide-js';
import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [
		paraglideVitePlugin({ project: './project.inlang', outdir: './src/paraglide', disableAsyncLocalStorage: true }),
		tailwindcss(),
		sveltekit()
	],
	server: {
		proxy: {
			'/v0': {
				target: 'http://localhost:8000',
				changeOrigin: true
			},
			'/docs': {
				target: 'http://localhost:8000',
				changeOrigin: true
			},
			'/swagger.json': {
				target: 'http://localhost:8000',
				changeOrigin: true
			}
		}
	}
});
