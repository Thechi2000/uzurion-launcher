import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import SettingsLogo from "./settings-logo";
import Accounts from "./accounts";
import Socials from "./socials";
import Status from "./status";
import Settings from "./settings";

function App() {
  const [settingsVisibility, setSettingsVisibility] = useState(false);

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
            <SettingsLogo setSettingsVisibility={setSettingsVisibility}/>
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

      <Settings visibility={settingsVisibility} setVisibility={setSettingsVisibility}/>
    </div>
    );
}

export default App;
