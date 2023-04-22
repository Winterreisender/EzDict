import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [description, setDescription] = useState("");
  const [queryWord, setQueryWord] = useState("");

  async function queryDict() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let resp = await invoke("query", { word: queryWord });
    setDescription(resp.descriptions);
  }

  return (
    <div className="container">
      <div className="row">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            queryDict();
          }}
        >
          <input
            id="greet-input"
            required
            onChange={(e) => setQueryWord(e.currentTarget.value)}
            placeholder="输入单词..."
          />
          <button type="submit">查询</button>
        </form>
      </div>
      
      <hr style={{width: "50%"}}></hr>
      <p>{description}</p>
    </div>
  );
}

export default App;
