import { Button } from "@/components/ui/button";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuRadioGroup,
    DropdownMenuRadioItem,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import type { ThemeSetting } from "@/bindings";
import { useThemeSetting } from "@/hooks/theme/useTheme";
import { Monitor, Moon, Sun } from "lucide-react";

const ICONS = {
    system: Monitor,
    light: Sun,
    dark: Moon,
} as const;

type Props = {
    onSelect: (setting: ThemeSetting) => void;
};

export default function ThemeToggle({ onSelect }: Props) {
    const setting = useThemeSetting();
    // 解決済みテーマではなく設定値で選ぶ。dark: バリアントは light/dark しか
    // 区別できず、System を表せない。
    const Icon = ICONS[setting];

    return (
        <DropdownMenu>
            <DropdownMenuTrigger
                render={
                    <Button variant="outline" size="icon">
                        <Icon className="h-[1.2rem] w-[1.2rem]" />
                        <span className="sr-only">Toggle theme</span>
                    </Button>
                }
            />
            <DropdownMenuContent align="end">
                <DropdownMenuRadioGroup
                    value={setting}
                    onValueChange={(value) => onSelect(value as ThemeSetting)}
                >
                    <DropdownMenuRadioItem value="light">
                        <Sun />
                        Light
                    </DropdownMenuRadioItem>
                    <DropdownMenuRadioItem value="dark">
                        <Moon />
                        Dark
                    </DropdownMenuRadioItem>
                    <DropdownMenuRadioItem value="system">
                        <Monitor />
                        System
                    </DropdownMenuRadioItem>
                </DropdownMenuRadioGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    );
}
