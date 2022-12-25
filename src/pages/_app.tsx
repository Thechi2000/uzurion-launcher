import type { AppProps } from "next/app";

import "../play.css"
import "../login.css"
import "../settings.css";
import "../style.css";
import "../App.css";

// This default export is required in a new `pages/_app.js` file.
export default function MyApp({ Component, pageProps }: AppProps) {
  return <Component {...pageProps} />;
}
