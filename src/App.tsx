import "./App.css";
import ThemeToggle from "./components/features/theme/toggle";
import useTheme from "@/hooks/theme/useTheme";

function App() {
  const setTheme = useTheme();

  return (
    <main className="bg-background text-foreground">
      <ThemeToggle onSelect={setTheme} />
    </main>
  );
}

export default App;
