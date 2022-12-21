export default function Settings({visibility, setVisibility}){

    function parameters(){
        return (
            <div className="settings-parameters">
                <div><input placeholder="Java path"/></div>
                <div><input placeholder="Java path"/></div>
            </div>  
        )
    }

    return (
        <div id="settings-canvas" style={{visibility: visibility ? "visible" : "hidden"}}>
            <div id="settings">
                {parameters()}
                <div id="settings-navigator">
                    <p>JVM</p>
                    <p>Launcher</p>
                    <p onClick={() => setVisibility(false)}>Quit</p>
                </div>
            </div>
        </div>
    )
}