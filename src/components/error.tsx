export default function Error(props: {name: string; description: string; hide: CallableFunction}){
    return (
        <div className="modal-window-canvas">
            <div id="error-modal-window" className="modal-window">
                <div>
                    <h2>{props.name}</h2>
                    <div><p>{props.description}</p></div>
                    <button onClick={() => props.hide()}>Ok</button>
                </div>
            </div>
        </div>
    )
}