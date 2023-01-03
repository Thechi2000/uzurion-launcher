import React from "react";
import ReactDOM from "react-dom/client";
import App from "./components/App";

import "./style/style.css"
import "./style/login.css"
import "./style/play.css"
import "./style/settings.css"
import "./style/error.css"

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
