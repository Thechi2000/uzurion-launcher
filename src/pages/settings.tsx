import { useEffect, useState } from "react"
import { debug } from "tauri-plugin-log-api"

export default function Settings({settings, setSettings, hide}){
    const [selectedPane, setSelectedPane] = useState("game")

    function updateGameValue(idx, value) {
        debug(`Updating game ${idx} to ${value}`)

        var set = Object.assign({}, settings)
        set["game"][idx] = value

        setSettings(set)
    }
    function updateResolution(value){
        value = value.split("x")
        updateGameValue("resolution", [parseInt(value[0]), parseInt(value[1])])
    }

    useEffect(() => {
        debug(JSON.stringify(settings))
        document.getElementById("resolution-input").setAttribute("value", `${settings['game']['resolution'][0]}x${settings['game']['resolution'][1]}`)
        document.getElementById("ram-input").setAttribute('value', settings['game']['ram'])
    })
    

    function parameters(){
        switch (selectedPane) {
            case "game":
                return (
                    <div className="settings-parameters">
                        <div>
                            <span className="setting-name">RAM: </span>
                            <input id="ram-input" type="range" onChange={e => updateGameValue('ram', parseInt(e.target.value))} min={1024} max={16384}/>
                        </div>
        
                        <div>
                            <span className="setting-name">Resolution:</span>
                            <select id="resolution-input" onChange={e => updateResolution(e.target.value)}>
                                <option value="1920x1080">1920x1080</option>
                                <option value="2560x1440">2560x1440</option>
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

    function selectedClass(name) {
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
                    <div id="settings-quit"><p onClick={hide}>Quit</p></div>
                </div>
            </div>
        </div>
    )
}