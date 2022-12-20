import refreshLogo from "../assets/refresh.svg"
import Image from "next/image";
import { useState } from "react";

export default function Status() {
    const [status, setStatus] = useState("offline")

    function statusText(){
        if(status == "offline") {
            return "Offline"
        } else {
            return "Online"
        }
    }

    function statusColor(){
        if (status == "offline") {
            return "red"
        } else {
            return "lime"
        }
    }

    function switchStatus(){
        if (status == "offline"){
            setStatus("online")
        } else {
            setStatus("offline")
        }
    }

    function getOnlinePlayers(){
        if (status == "offline"){
            return (
                <p></p>
            )
        } else {
            return (
                <p>4 players online</p>
            )
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
            <Image src={refreshLogo} onClick={() => switchStatus()} width="40%" height="40%"/>
        </div>
    )
}