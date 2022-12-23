import { invoke } from "@tauri-apps/api/tauri"
import { useState } from "react"

export default function MojangLogin({hide}){
    const [email, setEmail] = useState("")
    const [pwd, setPwd] = useState("")
    const [remember, setRemember] = useState(false)


    async function login() {
        await invoke(
            "mojang_login",
            {
                email: email,
                password: pwd,
                remember: remember,
            }
        )
    }

    return (
        <div id="mojang-login" className="vertical-container">
            <input id="email" placeholder="Email" onChange={e => setEmail(e.target.value)} value={email}/>
            <input id="password" type="password" placeholder="Password" onChange={e => setPwd(e.target.value)} value={pwd}/>
            <div className="container" style={{justifyContent:"flex-start"}}>
                <input id="remember-me-checkbox" type="checkbox" checked={remember}/>
                <span onClick={() => setRemember(!remember)}>Remember me</span>
            </div>
            <button onClick={() => login()}>Login</button>
            <button onClick={hide}>Cancel</button>
        </div>
    )
}