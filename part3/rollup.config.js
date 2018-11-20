import babel from "rollup-plugin-babel"
import uglify from "rollup-plugin-uglify"
import wasm from "rollup-plugin-wasm"

export default {
    input: './target/deploy/hunt.js',
    output: {
        name: 'hunt',
        file: './release/hunt.js',
        format: 'es',
    },
    plugins: [
        babel({
            exclude: 'node_modules/**'
        }),
        uglify
    ]
};