import React from 'react';
import logo from './logo.svg';
import './App.css';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        Tauri: {window.tauri ? "Yes" : "No"}
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          XDXD <code>src/App.js</code> and save to reload.
        </p>
        <button onClick={() => {
          window.tauri.invoke({
            cmd: "hello"
          })
        }}>invoke Hello</button>
      </header>
    </div>
  );
}

export default App;
