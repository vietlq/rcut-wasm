import * as wasm from "rcut-wasm";

function ready() {
    let result = wasm.rcut_char("🦃🐔🐓🐣🐤🐥🐦🐧🕊🦅🦆🦢🦉🦚🦜", "9,4,7,3,12,5-15");
    console.log(result);
    document.getElementById("content").innerText = result;
}

//document.addEventListener("DOMContentLoaded", ready);
//document.addEventListener("load", ready);
ready();
