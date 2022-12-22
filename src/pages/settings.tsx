import { useState } from "react"

export default function Settings({visibility, setVisibility}){
    const [selectedPane, setSelectedPane] = useState("JVM")

    function parameters(){
        switch (selectedPane) {
            case "JVM":
                return (
                    <div className="settings-parameters">
                        <div>
                            <span className="setting-name">RAM: </span>
                            <input type="range" min={1} max={16}/>
                        </div>
        
                        <div>
                            <span className="setting-name">Resolution:</span>
                            <select id="game-resolution">
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
        <div id="settings-canvas" style={{visibility: visibility ? "visible" : "hidden"}}>
            <div id="settings">
                {parameters()}
                <div id="settings-navigator">
                    <div id="settings-pages-selectors">
                        <div className={selectedClass("JVM")} onClick={() => setSelectedPane("JVM")}><p>JVM</p></div>
                        <div className={selectedClass("Launcher")} onClick={() => setSelectedPane("Launcher")}><p>Launcher</p></div>
                    </div>
                    <div id="settings-quit"><p onClick={() => setVisibility(false)}>Quit</p></div>
                </div>
            </div>
        </div>
    )
}