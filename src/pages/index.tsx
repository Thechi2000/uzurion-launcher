import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
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

  async function play() {
    await invoke("play")
  }

  return (
    <div id="app">
      <h1 id="title">Uzurion</h1>

      <div id="app-menu">
        <div className="vertical-container left">
          <Accounts/>
          <Settings/>
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
    );
}

export default App;
