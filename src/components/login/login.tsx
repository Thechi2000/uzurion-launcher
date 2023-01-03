import { invoke } from "@tauri-apps/api/tauri";
import { ReactElement, useState } from "react";
import MojangLogin from "./mojang-login";

export default function Login(props: {hide: CallableFunction}){
    const [loginPane, setLoginPane] = useState(null as ReactElement | null)

    const mojangLogin = <MojangLogin hide={() => setLoginPane(null)}/>;
    const loginSelection = (
        <div className="vertical-container">
            <button onClick={() => setLoginPane(mojangLogin)}>Mojang</button>
            <button onClick={() => invoke('microsoft_login')}>Microsoft</button>
            <button onClick={() => props.hide()}>Quit</button>
        </div>
    );

    return (
        <div id="login-window-canvas" className="modal-window-canvas">
            <div id="login-window" className="modal-window">
                {loginPane === null ? loginSelection : loginPane}
            </div>
        </div>
    )
}