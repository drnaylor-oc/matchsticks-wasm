// const CopyPlugin = require("copy-webpack-plugin");
const path = require("path");
const autoprefixer = require('autoprefixer')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

let config = {
    entry: {
        index: "./src/js/index.js",
        answer: "./src/js/answers.js"
    },
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "[name]1.js",
    },
    experiments: {
        asyncWebAssembly: true,
    },
    module: {
        rules: [
            {
                test: /\.((s)?css)$/,
                use: [
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
        new HtmlWebpackPlugin({ template: './src/index.html', filename: "index.html", chunks: ["index"] }),
        new HtmlWebpackPlugin({ template: './src/answer.html', filename: "answer.html", chunks: ["answer"] }),
    ]
};

module.exports = (env, argv) => {
    config.mode = argv.mode ?? 'development';
    if (argv.mode === 'production') {
        config.plugins.push(new MiniCssExtractPlugin());
        config.module.rules[0].use.unshift({
            loader: MiniCssExtractPlugin.loader
        });
    } else {
        config.module.rules[0].use.unshift({
            loader: "style-loader"
        });
    }

    return config;
}