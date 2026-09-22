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
  html: {
    // 既定では Rsbuild が独自の HTML を生成し、リポジトリ直下の index.html は
    // 使われない。初回描画前のテーマ適用スクリプトを載せるため明示する。
    template: './index.html',
  },
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
          // デスクトップアプリは配信がローカルのため分割の利点が無く、
          // コールド起動時に未生成の chunk を取りに行って失敗する経路だけが残る。
          autoCodeSplitting: false,
          routesDirectory: './src/routes',
          generatedRouteTree: './src/routeTree.gen.ts',
          routeFileIgnorePrefix: '-',
        }),
      ],
      watchOptions: {
        poll: 1000,
      },
      // 同上。単一バンドルにして chunk の取得そのものを無くす。
      optimization: {
        splitChunks: false,
        runtimeChunk: false,
      },
    },
  },
});
