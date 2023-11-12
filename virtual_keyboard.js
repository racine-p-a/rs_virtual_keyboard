import init, {Titi} from "./pkg/rs_virtual_keyboard.js";

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
        //greet(document.getElementById('charTables').value);
        //const universe = encodings('titi');
        //console.log(universe);
        const datum = new Titi(14);
        console.log(datum.get_start());
        console.log(datum.get_end());
    });
});

/*
init().then(() => {
    greet("titi");
});
*/