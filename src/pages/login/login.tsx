import { useState } from "react";
import MojangLogin from "./mojang-login";

export default function Login({hide}){
    const [loginPane, setLoginPane] = useState(undefined)

    const mojangLogin = <MojangLogin hide={() => setLoginPane(undefined)}/>;
    const loginSelection = (
        <div className="vertical-container">
            <button onClick={() => setLoginPane(mojangLogin)}>Mojang</button>
            <button onClick={() => window.location.replace("https://www.epfl.ch")}>Microsoft</button>
            <button onClick={hide}>Quit</button>
        </div>
    );

    return (
        <div id="login-window-canvas" className="modal-window-canvas">
            <div id="login-window" className="modal-window">
                {loginPane === undefined ? loginSelection : loginPane}
            </div>
        </div>
    )
}