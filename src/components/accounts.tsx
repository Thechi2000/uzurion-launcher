import accountLogo from "../assets/account.svg"

export default function Accounts(props: {show: CallableFunction}) {
    return (
        <div id="accounts">
            <img id="account-logo" src={accountLogo} width={60} height={60} onClick={() => props.show()}/>
        </div>
    )
}