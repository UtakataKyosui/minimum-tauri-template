import { useEffect, useSyncExternalStore } from "react";
import { commands, events, type ResolvedTheme, type ThemeSetting } from "@/bindings";

// 解決済みテーマは <html> のクラスだけが持つ。React の state に写すと、
// 値を読まないコンポーネントまで再レンダリングの対象になる。
// 設定値だけは UI（チェックマーク）が読むため、購読した側だけが
// 再レンダリングされるよう外部ストアとして持つ。
let currentSetting: ThemeSetting = "system";
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

function applyResolved(resolved: ResolvedTheme) {
    const root = document.documentElement;
    root.classList.remove("light", "dark");
    root.classList.add(resolved);
}

async function setTheme(next: ThemeSetting) {
    const resolved = await commands.setTheme(next);
    setSetting(next);
    applyResolved(resolved);
}

/** ドロップダウンのチェックマーク用。購読したコンポーネントだけが再レンダリングされる。 */
export function useThemeSetting(): ThemeSetting {
    return useSyncExternalStore(subscribe, () => currentSetting);
}

export default function useTheme() {
    useEffect(() => {
        let active = true;
        // 初期値の取得が終わる前に届いた themeChanged を取りこぼさないよう、
        // 購読を先に張る。後から解決した初期値で上書きもしない。
        let appliedFromEvent = false;

        const unlisten = events.themeChanged.listen((event) => {
            if (!active) return;
            appliedFromEvent = true;
            applyResolved(event.payload);
        });

        commands.getTheme().then((status) => {
            if (!active) return;
            setSetting(status.setting);
            if (!appliedFromEvent) applyResolved(status.resolved);
        });

        return () => {
            active = false;
            unlisten.then((dispose) => dispose());
        };
    }, []);

    return setTheme;
}
