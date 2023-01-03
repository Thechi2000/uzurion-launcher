import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import { debug, info, warn } from "tauri-plugin-log-api";

export default function Play(){
    const [updateStage, setUpdateStage] = useState(null as string | null)
    const [download, setDownload] = useState(null as string | null)
    const [doneDownload, setDoneDownload] = useState(0)
    const [totalDownload, setTotalDownload] = useState(1)

    async function play() {
        await invoke('check_update').then(() => invoke("play"))
    }

    function stageDescription(){
        switch(updateStage) {
            case 'fetching':
                return 'Fetching update'
            case 'cleaning':
                return 'Cleaning game folder'
            case 'downloading':
                return <span>Downloading<br/>{download}</span>
            default:
                null
        }
    }

    useEffect(() =>{
        info('Registering game-update listener')
        let unlistener = listen(
            'game-update', 
            (e: {payload: {type: string, name: string, done: number, total: number}}) => {
                debug('Received event game-update with payload ' + JSON.stringify(e.payload))

                switch (e.payload.type) {
                    case 'start':
                        setUpdateStage('fetching')
                        break

                    case 'fetch_done':
                        setUpdateStage('cleaning')
                        break

                    case 'clean_done':
                        setUpdateStage('downloading')
                        break

                    case 'update_state':
                        setDownload(e.payload.name)
                        setDoneDownload(e.payload.done)
                        setTotalDownload(e.payload.total)
                        break

                    case 'download_done':
                    case 'failure':
                        setUpdateStage(null)
                        setDownload(null)
                        setDoneDownload(0)
                        setTotalDownload(1)
                        break
                
                    default:
                        warn('Unknown payload')
                        break
                }

            }
        )

        return () => {
            info('Unregistering game-update listener')
            unlistener.then(u => u())
        }
    })

    return (
        <div id="play">
            <button id="play-button" style={{ display:updateStage === null ? 'inherit': 'none' }} onClick={() => play()}>Play</button>
            <div id="update" style={{ display:updateStage === null ? 'none': 'contents' }}>
                <div id="update-stage-description" className="vertical-container">
                    <div>{stageDescription()}</div>
                    <div id="update-stage-progress-bar" style={{display:updateStage === 'downloading' ? 'inherit': 'none'}}><div style={{width: `${doneDownload/totalDownload * 100}%`}}></div></div>
                </div>
            </div>
        </div>
    )
}