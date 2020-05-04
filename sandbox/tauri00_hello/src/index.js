
const button = document.getElementById("hello")
let t = 0;

button.onclick = (ev) => {
  button.innerHTML = String(t++);
}