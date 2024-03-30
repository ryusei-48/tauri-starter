import { useState } from "react";
import reactLogo from "./assets/react.svg";
import tailwindoLogo from './assets/tailwindcss.svg';
import shadcnUiLog from './assets/shadcn-ui.png';
import { invoke } from "@tauri-apps/api/tauri";
import { ModeToggle } from '@/components/mode-toggle';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import "./App.css";

export default function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="container py-5">
      <h1 className="w-full text-center text-2xl">Welcome to Tauri!</h1>
      <div className="flex justify-center items-center gap-x-3 my-3">
        Theme togle:<ModeToggle />
      </div>
      <div className="flex flex-wrap justify-center gap-3">
        <a className="w-[150px] inline-flex flex-col" href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="h-[100px]" alt="Vite logo" />
          <span className="w-full text-center text-lg mt-1">Vite</span>
        </a>
        <a className="w-[150px] inline-flex flex-col" href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="h-[100px]" alt="Tauri logo" />
          <span className="w-full text-center text-lg mt-1">TAURI</span>
        </a>
        <a className="w-[150px] inline-flex flex-col" href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="h-[100px]" alt="React logo" />
          <span className="w-full text-center text-lg mt-1">React</span>
        </a>
        <a className="w-[150px] inline-flex flex-col" href="https://tailwindcss.com" target="_blank">
          <img src={tailwindoLogo} className="h-[100px]" alt="tailwindcss logo" />
          <span className="w-full text-center text-lg mt-1">tailwindcss</span>
        </a>
        <a className="w-auto inline-flex flex-col" href="https://ui.shadcn.com" target="_blank">
          <img src={shadcnUiLog} className="h-[100px]" alt="shadcn/ui logo" />
        </a>
      </div>

      <p className="text-center pt-9 pb-2">Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="w-full text-center"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <Input
          id="greet-input"
          className="w-[300px] inline-block mr-3 dark:bg-slate-900"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <Button type="submit">Greet</Button>
      </form>

      <p className="text-center mt-3">{greetMsg}</p>
    </div>
  );
}