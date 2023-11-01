import init, { greet } from "./pkg/rs_virtual_keyboard.js";
init().then(() => {
    greet("titi");
});