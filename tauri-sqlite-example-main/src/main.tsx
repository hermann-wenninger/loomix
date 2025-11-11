import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import Actix from "./components/Actix";
import Keys from "./components/Keys";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
    <Actix />
    <Keys />
  </React.StrictMode>,
);
