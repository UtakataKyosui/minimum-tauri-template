import "./App.css";
import ThemeToggle from "./components/features/theme/toggle";
import useTheme from "@/hooks/theme/useTheme";

function App() {

  useTheme(); 
  return (
    <main className="bg-background text-foreground">
      <ThemeToggle />
    </main>
  );
}

export default App;
