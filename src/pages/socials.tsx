import Image from "next/image"
import discordLogo from "../assets/discord.svg"

export default function Socials(){
    return (
        <div id="socials" className="container">
            <div>
                <a href="https://www.epfl.ch" target="_blank" title="Join our Discord !"><Image src={discordLogo} width={60} height={60}/></a>
            </div>
        </div>
    )
}