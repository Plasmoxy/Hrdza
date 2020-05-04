import "regenerator-runtime/runtime"
import {open} from 'tauri/api/dialog'
import {emit} from 'tauri/api/event'

const button = document.getElementById("hello")
const fpath = document.getElementById("fpath")

let t = 0;

button.onclick = (ev) => {
  emit("test");
  button.innerHTML = String(t++);
}