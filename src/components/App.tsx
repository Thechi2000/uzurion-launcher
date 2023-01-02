import { useState } from "react";
import SettingsLogo from "./settings-logo";
import Accounts from "./accounts";
import Socials from "./socials";
import Status from "./status";
import Settings from "./settings";
import Login from "./login/login";
import Play from "./play";

export default function App() {
  const [modalWindow, setModalWindow] = useState(undefined)

  const settingsModalWindow = <Settings hide={() => setModalWindow(undefined)}/>
  const loginModalWindow = <Login hide={() => setModalWindow(undefined)}/>

  return (
    <div>
      <div id="app">
        <h1 id="title">Uzurion</h1>

        <div id="app-menu">
          <div className="vertical-container">
            <Accounts show={() => setModalWindow(loginModalWindow)}/>
            <SettingsLogo show={() => setModalWindow(settingsModalWindow)}/>
          </div>
          <div className="vertical-container center">
            <Play/>
            <Socials/>
          </div>
          <div className="vertical-container right">
            <Status/>
          </div>
        </div>
      </div>

      {modalWindow}
    </div>
    );
}
