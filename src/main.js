const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  
  let canvas = document.getElementById("canvas");
  let ctx = canvas.getContext("2d");
  setInterval(async () => {
    let a = await invoke("update_birds");
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.beginPath();
    for (let i = 0; i < a.length; i++) {
      ctx.arc(a[i][0], a[i][1], 1, 0, 2 * Math.PI);
      ctx.fillStyle = 'red';
      ctx.stroke();
    }
    
  }, 10);
}

async function make_bird() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("make_birds");
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document
    .querySelector("#greet-button")
    .addEventListener("click", () => greet());
  document
    .querySelector("#make-button")
    .addEventListener("click", () => make_bird());
});
