import * as wasm from "rcut-wasm";

function process_raw_array(raw_array, char_ranges) {
    return wasm.rcut_chars_from_raw(raw_array, raw_array.length, char_ranges);
}

function show_rcut() {
    let birds = "🦃🐔🐓🐣🐤🐥🐦🐧🕊🦅🦆🦢🦉🦚🦜";
    let char_ranges = "9,4,7,3,12,5-15";

    let unmerged_result = wasm.rcut_chars(birds, char_ranges);
    console.log(unmerged_result);

    let merged_result = wasm.rcut_chars(birds, char_ranges, true);
    console.log(merged_result);

    let unmerged_bytes = wasm.rcut_bytes(birds, char_ranges);
    let merged_bytes = wasm.rcut_bytes(birds, char_ranges);

    document.getElementById("content").innerText = [
        unmerged_result,
        merged_result,
        unmerged_bytes,
        merged_bytes].join('');
}

window.rcut = {};
window.rcut.process_raw_array = process_raw_array;
window.rcut.show_rcut = show_rcut;

//document.addEventListener("DOMContentLoaded", ready);
//document.addEventListener("load", ready);
//show_rcut();
