import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

const handleIsTauri = () => {
  return Boolean(
    typeof window !== 'undefined' && window !== undefined
    && window.__TAURI_IPC__ !== undefined
  )
}

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [windowName, setWindowName] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  async function getWindow() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setWindowName(await invoke("get_window_name"));
  }

  useEffect(()=>{
    const interval = setInterval(()=> {getWindow()}, 3000);
    return ()=>{
      clearInterval(interval);
    }


  },[])
  return handleIsTauri() ? (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>
      <button type="submit" onClick={()=>{getWindow()}}>refresh</button>
    <span> window name {windowName}</span>
      <div className="row">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            greet();
          }}
        >
          <input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <button type="submit">Greet</button>
        </form>
      </div>
      <p>{greetMsg}</p>
    </div>
  ): (
  <h1>Unable to run outside of tauri</h1>);
}

export default App;
