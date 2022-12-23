export default function Login({hide}){
    return (
        <div id="login-window-canvas" className="modal-window-canvas">
            <div id="login-window" className="modal-window">
                <div className="vertical-container">
                    <button>Mojang</button>
                    <button>Microsoft</button>
                    <button onClick={hide}>Quit</button>
                </div>
            </div>
        </div>
    )
}