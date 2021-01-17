const path = require("path");
const glob = require("glob");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const PurgecssPlugin = require("purgecss-webpack-plugin");
const CssoWebpackPlugin = require("csso-webpack-plugin").default;

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
        status: "./src/pages/status/index.js",
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
        new PurgecssPlugin({
            paths: glob.sync(`${path.join(__dirname, "public")}/**/*`, { nodir: true }),
            safelist: ["flash-animation"]
        }),
        new CssoWebpackPlugin(),
        new HtmlWebpackPlugin({
            filename: "home.html",
            template: "./public/home.html",
            chunks: ["common", "home"]
        }),
        new HtmlWebpackPlugin({
            filename: "config_cat.html",
            template: "./public/config_cat.html",
            chunks: ["common", "config_cat"]
        }),
        new HtmlWebpackPlugin({
            filename: "config_num.html",
            template: "./public/config_num.html",
            chunks: ["common", "config_num"],
        }),
        new HtmlWebpackPlugin({
            filename: "run.html",
            template: "./public/run.html",
            chunks: ["common", "run"]
        }),
        new HtmlWebpackPlugin({
            filename: "results.html",
            template: "./public/results.html",
            chunks: ["common", "results"]
        }),
        new HtmlWebpackPlugin({
            filename: "respond_cat.html",
            template: "./public/respond_cat.html",
            chunks: ["common", "respond_cat"]
        }),
        new HtmlWebpackPlugin({
            filename: "respond_num.html",
            template: "./public/respond_num.html",
            chunks: ["common", "respond_num"]
        }),
        new HtmlWebpackPlugin({
            filename: "status.html",
            template: "./public/status.html",
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
