import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Image from "next/image";
import reactLogo from "../assets/react.svg";
import tauriLogo from "../assets/tauri.svg";
import nextLogo from "../assets/next.svg";
import Settings from "./settings";
import Accounts from "./accounts";
import Socials from "./socials";
import Status from "./status";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="container">
      <h1>Uzurion</h1>

      <div id="app-menu">
        <div className="vertical-container">
          <Accounts/>
          <Settings/>
        </div>
        <div className="vertical-container center">
          <button>Play</button>
          <Socials/>
        </div>
        <div className="vertical-container">
          <Status/>
        </div>
      </div>
    </div>
    );
}

export default App;
