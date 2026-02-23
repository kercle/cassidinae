import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

const BACKEND_HOST = process.env.BACKEND_HOST || 'localhost:3000';

export default defineConfig({
	plugins: [
		tailwindcss(),
		sveltekit(),
		wasm(),
		topLevelAwait()
	],
	server: {
		port: 5173,
		proxy: {
			'/ws': { target: `ws://${BACKEND_HOST}`, ws: true }
		}
	}
});
