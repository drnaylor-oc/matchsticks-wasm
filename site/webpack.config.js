// const CopyPlugin = require("copy-webpack-plugin");
const path = require("path");
const autoprefixer = require('autoprefixer')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const devMode = process.env.NODE_ENV !== "production";

module.exports = {
    entry: {
        index: "./src/js/index.js",
        answer: "./src/js/answers.js"
    },
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "[name]1.js",
    },
    mode: "development",
    experiments: {
        asyncWebAssembly: true,
    },
    module: {
        rules: [
            {
                test: /\.((s)?css)$/,
                use: [
                    {
                        // Adds CSS to the DOM by injecting a `<style>` tag (in dev)
                        loader: devMode ? "style-loader" : MiniCssExtractPlugin.loader
                    },
                    {
                        // Interprets `@import` and `url()` like `import/require()` and will resolve them
                        loader: 'css-loader'
                    },
                    {
                        // Loader for webpack to process CSS with PostCSS
                        loader: 'postcss-loader',
                        options: {
                            postcssOptions: {
                                plugins: [
                                    autoprefixer
                                ]
                            }
                        }
                    },
                    {
                        // Loads a SASS/SCSS file and compiles it to CSS
                        loader: 'sass-loader'
                    }
                ]
            }
        ]
    },
    plugins: [
        // new CopyPlugin({
        //     patterns: [{ from: "./src/index.html" }],
        // }),
        new HtmlWebpackPlugin({ template: './src/index.html', filename: "index.html", chunks: ["index"] }),
        new HtmlWebpackPlugin({ template: './src/answer.html', filename: "answer.html", chunks: ["answer"] }),
    ].concat(devMode ? [] : [new MiniCssExtractPlugin()]),
};
