import accountLogo from "../assets/account.svg"

export default function Accounts({show}) {
    return (
        <div id="accounts">
            <img id="account-logo" src={accountLogo} width={60} height={60} onClick={show}/>
        </div>
    )
}