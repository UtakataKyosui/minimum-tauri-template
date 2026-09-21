# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Use-Packages Docs-Site Memo

### フロントエンド

| 分野 | 名前 | 選定理由 | URL | 
| --- | --- | --- | --- |
| ルーティング | TanStack Router | 扱いやすいルーティングライブラリ | https://tanstack.com/router/latest/docs/overview | 
| ビルドツール | Rsbuild | Viteよりビルドが早い | https://rsbuild.rs/ | 
| コード整形ツール | Biome | Linter + Formatterが統合されている | https://biomejs.dev/ | 
| CSSライブラリ | Tailwind CSS | ユーティリティクラスでのCSS宣言 | https://tailwindcss.com/ | 
| UIコンポーネント | shadcn/ui | Tailwind CSS + Base UIで構築されるUIコンポーネント集 | https://ui.shadcn.com/ | 

### ネイティブのロジック

| 分野 | 名前 | 選定理由 | URL | 
| --- | --- | --- | --- |
| ネイティブアプリのフレームワーク | Tauri | フロントエンドの技術で画面を作って、Rustでロジックを開発する | https://v2.tauri.app/ | 
| 
| Rustの型をエクスポートするクレート | specta | | https://specta.dev/docs/specta | 
| TauriのコマンドをTypeScriptに公開する | tauri-specta | | https://specta.dev/docs/tauri-specta | 