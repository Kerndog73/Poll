const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = {
    entry: {
        common: "./src/pages/common/index.js",
        home: "./src/pages/home/index.js",
        config_cat: "./src/pages/config_cat/index.js",
        config_num: "./src/pages/config_num/index.js",
        run: "./src/pages/run/index.js",
        results: "./src/pages/results/index.js",
        respond_cat: "./src/pages/respond_cat/index.js",
        respond_num: "./src/pages/respond_num/index.js",
    },
    output: {
        filename: "js/[name].js",
        publicPath: "/",
        path: path.resolve(__dirname, "dist"),
    },
    plugins: [
        new MiniCssExtractPlugin({
            filename: "css/[name].css"
        }),
        new HtmlWebpackPlugin({
            filename: "home.html",
            template: "./public/home.html",
            chunks: ["common", "home"]
        }),
        new HtmlWebpackPlugin({
            filename: "config_cat.html",
            template: "./public/config_cat.html",
            chunks: ["common"]
        }),
        new HtmlWebpackPlugin({
            filename: "config_num.html",
            template: "./public/config_num.html",
            chunks: ["common"],
            base: "/"
        }),
        new HtmlWebpackPlugin({
            filename: "run.html",
            template: "./public/run.html",
            chunks: ["common"]
        }),
        new HtmlWebpackPlugin({
            filename: "results.html",
            template: "./public/results.html",
            chunks: ["common"]
        }),
        new HtmlWebpackPlugin({
            filename: "respond_cat.html",
            template: "./public/respond_cat.html",
            chunks: ["common"]
        }),
        new HtmlWebpackPlugin({
            filename: "respond_num.html",
            template: "./public/respond_num.html",
            chunks: ["common"]
        }),
    ],
    module: {
        rules: [
            {
                test: /\.s[ac]ss$/i,
                use: [
                    MiniCssExtractPlugin.loader,
                    "css-loader",
                    "sass-loader",
                ],
            },
        ],
    },
};
