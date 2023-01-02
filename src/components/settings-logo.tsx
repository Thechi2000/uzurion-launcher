import settingsLogo from "../assets/settings.svg"

export default function SettingsLogo({show}){
    return (
        <div id="settings-logo">
            <img id="settings-logo-image" src={settingsLogo} width={60} height={60} onClick={show}/>
        </div>
    )
}