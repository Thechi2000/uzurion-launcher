import { ReactElement, useEffect, useState } from "react";
import SettingsLogo from "./settings-logo";
import Accounts from "./accounts";
import Socials from "./socials";
import Status from "./status";
import Settings from "./settings";
import Login from "./login/login";
import Play from "./play";
import Error from "./error"
import { listen } from "@tauri-apps/api/event";
import { debug } from "tauri-plugin-log-api";

export default function App() {
  const [modalWindow, setModalWindow] = useState(null as ReactElement | null)

  const settingsModalWindow = <Settings hide={() => setModalWindow(null)}/>
  const loginModalWindow = <Login hide={() => setModalWindow(null)}/>

  useEffect(() =>{
    let u = listen('error', (e: {payload: {name: string; description: string}}) => {
      debug("Received error with payload: " + JSON.stringify(e.payload))
      return setModalWindow(<Error name={e.payload.name} description={e.payload.description} hide={() => setModalWindow(null)} />);
    })
    return () => {u.then(u => u())}
  })

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
