/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        fontFamily: {
            kode: ["Kode Mono", "sans-serif"]
        },
    },
    plugins: [],
}