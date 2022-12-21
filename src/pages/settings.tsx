export default function Settings(){
    function parameters(){
        return (
            <div className="settings-parameters">
                <div><input placeholder="Java path"/></div>
                <div><input placeholder="Java path"/></div>
            </div>  
        )
    }

    return (
        <div id="settings-canvas">
            <div id="settings">
                {parameters()}
                <div id="settings-navigator">
                    <p>JVM</p>
                    <p>Launcher</p>
                </div>
            </div>
        </div>
    )
}