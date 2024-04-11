/** @type {import('tailwindcss').Config} */
module.exports = {
    // TODO: read about jit mode
    // mode: 'jit',
    darkMode: 'class',
    content: {
        files: ['*.html', './src/**/*.rs', './input.css'],
    },
    theme: {
        extend: {
            fontFamily: {
                opensans: ['Open Sans', 'sans-serif'],
                robotomono: ['Roboto Mono', 'monospace'],
            },
        },
    },
    plugins: [],
};
