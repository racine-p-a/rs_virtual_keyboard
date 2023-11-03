import init, { greet } from "./pkg/rs_virtual_keyboard.js";

// Fonctionne
/*
document.getElementById('boutonTest').addEventListener('click', function () {
    init().then(() => {
        greet("test");
    })
});
*/

init().then(() => {
    document.getElementById('boutonTest').addEventListener('click', function () {

        greet(document.getElementById('charTables').value);
    });
});

/*
init().then(() => {
    greet("titi");
});
*/