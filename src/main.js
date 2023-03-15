const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {

  let canvas = document.getElementById("canvas");
  let ctx = canvas.getContext("2d");
  
  setInterval(async () => {
    let a = await invoke("update_birds");
    ctx.canvas.width  = 1000;
    ctx.canvas.height = 1000;


    for(let i = 0; i < a.length; i++) {
      let count = 0;
      for(let j = 0; j < a.length; j++) {
        if(i != j) {
          let dx = a[i][0] - a[j][0];
          let dy = a[i][1] - a[j][1];
          let dist = Math.sqrt(dx*dx + dy*dy);
          if(dist < 20) {
            count++;
            ctx.beginPath();
            ctx.moveTo(a[i][0], a[i][1]);
            ctx.lineTo(a[j][0], a[j][1]);
            ctx.strokeStyle = `rgba(1,1,1, ${0.01*count})`;
            ctx.stroke();
          }
        }
      }

      ctx.beginPath();
      ctx.arc(a[i][0], a[i][1], 1, 0, 2 * Math.PI);
      ctx.fillStyle = `rgb(${count*10},0,0)`;
      ctx.fill();


     
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
