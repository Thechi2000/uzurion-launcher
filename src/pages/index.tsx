import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import SettingsLogo from "./settings-logo";
import Accounts from "./accounts";
import Socials from "./socials";
import Status from "./status";
import Settings from "./settings";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  async function play() {
    await invoke("play")
  }

  return (
    <div>
      <div id="app">
        <h1 id="title">Uzurion</h1>

        <div id="app-menu">
          <div className="vertical-container left">
            <Accounts/>
            <SettingsLogo/>
          </div>
          <div className="vertical-container center">
            <button id="play" onClick={() => play()}>Play</button>
            <Socials/>
          </div>
          <div className="vertical-container right">
            <Status/>
          </div>
        </div>
      </div>

      <Settings/>
    </div>
    );
}

export default App;
