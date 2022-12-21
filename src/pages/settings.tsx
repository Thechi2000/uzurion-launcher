import Image from "next/image"
import settingsLogo from "../assets/settings.svg"

export default function Settings(){
    return (
        <div id="settings">
            <Image id="settings-logo" src={settingsLogo} width={60} height={60}/>
        </div>
    )
}