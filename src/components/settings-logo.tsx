import settingsLogo from "../assets/settings.svg"

export default function SettingsLogo(props: {show: CallableFunction}){
    return (
        <div id="settings-logo">
            <img id="settings-logo-image" src={settingsLogo} width={60} height={60} onClick={() => props.show()}/>
        </div>
    )
}