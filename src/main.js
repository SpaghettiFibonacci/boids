const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  
  let canvas = document.getElementById("canvas");
  
  setInterval(async () => {
    let ctx = canvas.getContext("2d");
    let a = await invoke("update_birds");
    ctx.clearRect(0, 0, canvas.width, canvas.height);
 
    ctx.canvas.width  = 500;
    ctx.canvas.height = 500;
    
    for (let i = 0; i < a.length; i++) {
      ctx.beginPath();
      ctx.arc(a[i][0], a[i][1], 3, 0, 2 * Math.PI);
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
