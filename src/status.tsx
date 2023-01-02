import refreshLogo from "./assets/refresh.svg"
import { useState, useEffect } from "react";
import { listen, emit } from '@tauri-apps/api/event'
import { appWindow, WebviewWindow } from '@tauri-apps/api/window'
import { debug, trace } from 'tauri-plugin-log-api'


export default function Status() {
    const [status, setStatus] = useState(undefined)
    
    useEffect(() => {
        let unlistener = listen("server-status", (e) => {
            debug("Received server-status event with payload " + JSON.stringify(e.payload))
            setStatus(e.payload)
        })

        return () => {
            trace("Unregistering status listener")
            unlistener.then(u => u())
        }
    }, []);
    
    function isOnline(){
        return !(status == undefined || !status["online"])
    }

    function statusText(){
        if(isOnline()) {
            return "Online"
        } else {
            return "Offline"
        }
    }

    function statusColor(){
        if (isOnline()) {
            return "lime"
        } else {
            return "red"
        }
    }

    function getOnlinePlayers(){
        if (isOnline()){
            return status["players"]["online"]
        } else {
            return undefined
        }
    }

    return (
        <div id="status">
            <h2>Status</h2>
            <div className="container">
                <p>{statusText()}</p>
                <svg viewBox="0 0 20 20" width="7%" height="7%"><circle id="Ellipse_8" cx="10" cy="10" r="10" fill={statusColor()} /></svg>
            </div>
            {getOnlinePlayers()}
            <div><img src={refreshLogo} onClick={() => emit("refresh-server-status")} width="40%" height="40%"/></div>
        </div>
    )
}