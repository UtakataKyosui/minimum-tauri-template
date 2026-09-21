import { defineConfig } from '@rsbuild/core';
import { pluginReact } from '@rsbuild/plugin-react';
import {tanstackRouter} from '@tanstack/router-plugin/rspack';
import { pluginTailwindcss } from '@rsbuild/plugin-tailwindcss';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [pluginReact(), pluginTailwindcss()],
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
    rspack: {
        plugins: [
            tanstackRouter({
                target: "react",
                autoCodeSplitting: true,
                routesDirectory: "./src/routes",
                generatedRouteTree: "./src/routeTree.gen.ts",
                routeFileIgnorePrefix: "-"
            })
        ]
    }
  }
});