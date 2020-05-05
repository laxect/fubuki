const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
    mode: "production",
    module: {
        rules: [
            {
                test: /\.sass$/,
                use: [
                    "style-loader",
                    "css-loader",
                    {
                        loader: 'sass-loader',
                        options: {
                            implementation: require('sass')
                        }
                    }
                ]
            }
        ]
    },
    entry: {
        index: "./index.js"
    },
    output: {
        path: dist,
        filename: "index.js"
    },
    devServer: {
        contentBase: dist,
    },
    plugins: [
        new CopyPlugin([
            path.resolve(__dirname, "public")
        ]),

        new WasmPackPlugin({
            crateDirectory: "fubuki",
        }),
    ]
};
