import React from 'react';
import logo from './logo.svg';
import {emit} from 'tauri/api/event'
import './App.css';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        {window.tauri ? "Yes" : "No"}
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <button onClick={() => {
          emit("test", "")
        }}>Emit "test"</button>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
