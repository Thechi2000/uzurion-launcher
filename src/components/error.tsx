export default function Error(props: {name: string; description: string; hide: CallableFunction}){
    return (
        <div className="modal-window-canvas">
            <div className="modal-window">
                <h1>{props.name}</h1>
                <p>{props.description}</p>
                <button onClick={() => props.hide()}>Ok</button>
            </div>
        </div>
    )
}