const webpack = require('webpack');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const path = require('path');

// this is how to access ...
module.exports = (env, args) => {
    const isProductionMode = (args.mode === 'production');
    const buildpath = isProductionMode ?  'docs' : 'build';

    return {
        entry: './index.js',
        output: {
            // using hashes make sure that old cashes are succcesfully taken care off [contentHash]
            path: path.resolve(__dirname, buildpath),
            filename: isProductionMode ? '[name].js' : '[name].[hash].js',
        },
        plugins: [
            new HtmlWebpackPlugin({
                template: 'index.html'
            }),
            new WasmPackPlugin({
                crateDirectory: path.resolve(__dirname, '.')
            }),
            new webpack.ProvidePlugin({
                TextDecoder: ['text-encoding', 'TextDecoder'],
                TextEncoder: ['text-encoding', 'TextEncoder']
            })
        ]
    }
}