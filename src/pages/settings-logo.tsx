import Image from "next/image"
import settingsLogo from "../assets/settings.svg"

export default function SettingsLogo({setSettingsVisibility}){
    return (
        <div id="settings-logo">
            <Image src={settingsLogo} width={60} height={60} onClick={() => setSettingsVisibility(true)}/>
        </div>
    )
}