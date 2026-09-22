import { useEffect } from "react";
import {commands} from "@/bindings";

export default function useTheme() {
    useEffect(() => {
        const root = document.documentElement;
        root.classList.remove("light", "dark");
        commands.getTheme().then((theme) => {
            root.classList.add(theme.resolved);
        });
    }, [])
}