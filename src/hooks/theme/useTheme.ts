import { useEffect, useSyncExternalStore } from "react";
import { commands, type ThemeSetting } from "@/bindings";

// 解決済みテーマは <html> のクラスだけが持つ。React の state に写すと、
// 値を読まないコンポーネントまで再レンダリングの対象になる。
// 設定値だけは UI（チェックマーク）が読むため、購読した側だけが
// 再レンダリングされるよう外部ストアとして持つ。
let currentSetting: ThemeSetting = "system";
// ユーザーが明示的に setTheme を呼んだら true にする。起動時の
// getTheme() は非同期なので、選択後に解決すると古い値で上書きしうる。
// このフラグが立った後は起動時の取得結果を適用しない。
let settingOverridden = false;
const listeners = new Set<() => void>();

function subscribe(listener: () => void) {
    listeners.add(listener);
    return () => {
        listeners.delete(listener);
    };
}

function setSetting(next: ThemeSetting) {
    if (currentSetting === next) return;
    currentSetting = next;
    for (const listener of listeners) listener();
}

const darkQuery = window.matchMedia("(prefers-color-scheme: dark)");

// 解決済みテーマは webview の prefers-color-scheme から読む。
// Rust の set_theme がウィンドウ外観を確定させると、System / Light / Dark の
// いずれでもこの値が正解になる。Rust へ問い合わせると、外観の反映前に
// 答えが返って System のとき古い値を掴む。
function applyResolved() {
    const root = document.documentElement;
    root.classList.remove("light", "dark");
    root.classList.add(darkQuery.matches ? "dark" : "light");
}

async function setTheme(next: ThemeSetting) {
    // await の前にフラグを立てる。await 中に起動時の getTheme() が
    // 解決してもユーザーの選択を上書きさせないため。
    settingOverridden = true;
    // 保存とウィンドウ外観の指定は Rust が担う。解決済みテーマは
    // 反映後に darkQuery から読む。
    await commands.setTheme(next);
    setSetting(next);
    applyResolved();
}

/** ドロップダウンのチェックマーク用。購読したコンポーネントだけが再レンダリングされる。 */
export function useThemeSetting(): ThemeSetting {
    return useSyncExternalStore(subscribe, () => currentSetting);
}

export default function useTheme() {
    useEffect(() => {
        let active = true;

        applyResolved();
        darkQuery.addEventListener("change", applyResolved);

        commands.getTheme().then((setting) => {
            // ユーザーがこの解決前に手動で選んでいたら、起動時の値で上書きしない。
            if (active && !settingOverridden) setSetting(setting);
        });

        return () => {
            active = false;
            darkQuery.removeEventListener("change", applyResolved);
        };
    }, []);

    return setTheme;
}
