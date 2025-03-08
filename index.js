import init, { transliterate } from './pkg/rust_wasm_indic.js';

async function run() {
    await init();

    const input = document.getElementById('input');
    const output = document.getElementById('output');
    const button = document.getElementById('transliterate');

    button.addEventListener('click', () => {
        const inputText = input.value;
        const result = transliterate(inputText, 'devanagari');
        output.value = result;
    });
}

run();
