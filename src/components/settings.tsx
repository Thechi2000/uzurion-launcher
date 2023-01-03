import { listen } from "@tauri-apps/api/event"
import { invoke } from "@tauri-apps/api/tauri"
import { useEffect, useState } from "react"
import { debug, error, trace } from "tauri-plugin-log-api"
import { useEffectOnce } from "usehooks-ts"

interface settings {
    game: {
        resolution: [number, number];
        ram: number
    };
    launcher: {}
}

export default function Settings(props: {hide: CallableFunction}){
    const [selectedPane, setSelectedPane] = useState("game")
    const [settings, setSettings] = useState({
        game: {
            resolution: [1920, 1080],
            ram: 1024
        },
        launcher: {}
    } as settings)
    const [settingsLoaded, setSetingsLoaded] = useState(false)

    const [ram, setRam] = useState(1024)
    const [resolution, setResolution] = useState([1920, 1080])

    async function updateSetting(updater: (set: settings) => void) {
        var set = Object.assign({}, settings);
        updater(set);
        await invoke("set_settings", {settings: set}).catch(e => error(e))
    }

    useEffect(() => {
        if(settingsLoaded) {
            setResolution(settings.game.resolution)
            setRam(settings.game.ram)
        }
    }, [settings, settingsLoaded])

    function genResolutionOptions(values: [number, number][]){
        return values.map(r => {
            let str = `${r[0]}x${r[1]}`
            return <option value={str}>{str}</option>
        })
    }


    useEffect(() => {
        trace("Registering settings listener")

        let unlistener = listen('settings-update', (e: {payload: settings}) => {
            debug(`Updating settings to ${JSON.stringify(e.payload)}`)
            setSettings(e.payload)
        })

        return () => {
            trace("Unregistering settings-update listener")
            unlistener.then(u => u())
        }
    })

    useEffectOnce(() => {
        invoke('get_settings').then(() => setSetingsLoaded(true))
    })
    

    function parameters(){
        switch (selectedPane) {
            case "game":
                return (
                    <div className="settings-parameters">
                        <div>
                            <span className="setting-name">RAM: </span>
                            <input value={ram} type="range" onChange={e => updateSetting(set => {set.game.ram=parseInt(e.target.value)})} min={1024} max={16384}/>
                            <input value={ram} type="number" onChange={e => updateSetting(set => {set.game.ram=parseInt(e.target.value)})} min={1024} max={16384}/>
                        </div>
        
                        <div>
                            <span className="setting-name">Resolution:</span>
                            <select id="resolution-input" value={resolution.map(p=>p.toString()).join('x')} onChange={e => updateSetting(set => set.game.resolution=e.target.value.split('x').map(i => parseInt(i)) as [number, number])}>
                                {genResolutionOptions([
                                    [1920, 1080],
                                    [2560, 1440],
                                ])}
                            </select>
                        </div>
                    </div>  
                )
            case "Launcher":
                return(
                    <div className="settings-parameters">
                        <p>Nothing so far :)</p>
                    </div>
                )
        }
        
    }

    function selectedClass(name: string) {
        return selectedPane == name ? "selected-settings-pane" : ""
    }

    return (
        <div id="settings-canvas" className="modal-window-canvas">
            <div id="settings" className="modal-window">
                {parameters()}
                <div id="settings-navigator">
                    <div id="settings-pages-selectors">
                        <div className={selectedClass("game")} onClick={() => setSelectedPane("game")}><p>Game</p></div>
                        <div className={selectedClass("launcher")} onClick={() => setSelectedPane("Launcher")}><p>Launcher</p></div>
                    </div>
                    <div id="settings-quit"><p onClick={() => props.hide()}>Quit</p></div>
                </div>
            </div>
        </div>
    )
}