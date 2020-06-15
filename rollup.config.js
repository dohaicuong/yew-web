import babel from "rollup-plugin-babel"
import uglify from "rollup-plugin-uglify"

export default {
  input: './target/deploy/main.js',
  output: {
    name: 'main',
    file: './release/main.js',
    format: 'es',
  },
  plugins: [
    babel({
      exclude: 'node_modules/**'
    }),
    uglify
  ]
}