import path from 'node:path';
import { defineConfig } from '@rsbuild/core';
import { pluginReact } from '@rsbuild/plugin-react';
import { tanstackRouter } from '@tanstack/router-plugin/rspack';
import { pluginTailwindcss } from '@rsbuild/plugin-tailwindcss';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  source: {
    // This project follows the Vite convention (`main.tsx`), while Rsbuild
    // defaults to looking for `src/index.tsx`.
    entry: {
      index: './src/main.tsx',
    },
  },
  plugins: [pluginReact(), pluginTailwindcss()],
  resolve: {
    alias: {
      '@': path.resolve(process.cwd(), './src'),
    },
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? {
        protocol: 'ws',
        host,
        port: 1421,
    } : undefined,
    watch: {
        ignored: ['**/node_modules/**', '**/.git/**',"**/src-tauri/**"]
    }
  },
  tools: {
    // Some systems impose a low native file-watcher limit. Polling keeps the
    // development server reliable in those environments.
    rspack: {
      plugins: [
        tanstackRouter({
          target: 'react',
          autoCodeSplitting: true,
          routesDirectory: './src/routes',
          generatedRouteTree: './src/routeTree.gen.ts',
          routeFileIgnorePrefix: '-',
        }),
      ],
      watchOptions: {
        poll: 1000,
      },
    },
  },
});
