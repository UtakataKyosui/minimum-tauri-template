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

type Props = {
    onSelect: (setting: ThemeSetting) => void;
};

export default function ThemeToggle({ onSelect }: Props) {
    const setting = useThemeSetting();

    return (
        <DropdownMenu>
            <DropdownMenuTrigger
                render={
                    <Button variant="outline" size="icon" className="relative">
                        <Sun className="h-[1.2rem] w-[1.2rem] scale-100 rotate-0 transition-all dark:scale-0 dark:-rotate-90" />
                        <Moon className="absolute h-[1.2rem] w-[1.2rem] scale-0 rotate-90 transition-all dark:scale-100 dark:rotate-0" />
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
