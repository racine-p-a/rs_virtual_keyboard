import init, {Encoding} from "./pkg/rs_virtual_keyboard.js";

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
        /*
        const datum = new Encoding(13);
        console.log(datum.get_start());
        console.log(datum.get_end());*/
        console.log(Encoding.parcours('latin'));
    });
});

/*
init().then(() => {
    greet("titi");
});
*/