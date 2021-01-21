const path = require("path");
const glob = require("glob");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const PurgecssPlugin = require("purgecss-webpack-plugin");
const CssoWebpackPlugin = require("csso-webpack-plugin").default;
const HtmlWebpackSkipAssetsPlugin = require("html-webpack-skip-assets-plugin").HtmlWebpackSkipAssetsPlugin;

const TEMPLATE_PARAMS = {
    description: "Create a simple one-question poll in seconds with no signup required"
};

module.exports = {
    entry: {
        common: "./src/pages/common/index.js",
        config_cat: "./src/pages/config_cat/index.js",
        config_num: "./src/pages/config_num/index.js",
        run: "./src/pages/run/index.js",
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
        new PurgecssPlugin({
            paths: glob.sync(`${path.join(__dirname, "public")}/**/*`, { nodir: true }),
            safelist: ["flash-animation", "text-danger"]
        }),
        new CssoWebpackPlugin(),
        new HtmlWebpackPlugin({
            filename: "home.html",
            template: "./public/home.html",
            chunks: ["common"],
            templateParameters: TEMPLATE_PARAMS,
        }),
        new HtmlWebpackPlugin({
            filename: "config_cat.html",
            template: "./public/config_cat.html",
            chunks: ["common", "config_cat"],
            templateParameters: TEMPLATE_PARAMS,
        }),
        new HtmlWebpackPlugin({
            filename: "config_num.html",
            template: "./public/config_num.html",
            chunks: ["common", "config_num"],
            templateParameters: TEMPLATE_PARAMS,
        }),
        new HtmlWebpackPlugin({
            filename: "run.html",
            template: "./public/run.html",
            chunks: ["common", "run"],
            templateParameters: TEMPLATE_PARAMS,
        }),
        new HtmlWebpackPlugin({
            filename: "results_cat.html",
            template: "./public/results_cat.html",
            chunks: ["common"],
            templateParameters: TEMPLATE_PARAMS,
        }),
        new HtmlWebpackPlugin({
            filename: "results_num.html",
            template: "./public/results_num.html",
            chunks: ["common"],
            templateParameters: TEMPLATE_PARAMS,
        }),
        new HtmlWebpackPlugin({
            filename: "respond_cat.html",
            template: "./public/respond_cat.html",
            chunks: ["common"],
            templateParameters: TEMPLATE_PARAMS,
        }),
        new HtmlWebpackPlugin({
            filename: "respond_num.html",
            template: "./public/respond_num.html",
            chunks: ["common", "respond_num"],
            templateParameters: TEMPLATE_PARAMS,
        }),
        new HtmlWebpackPlugin({
            filename: "status.html",
            template: "./public/status.html",
            chunks: ["common"],
            templateParameters: TEMPLATE_PARAMS,
        }),
        new HtmlWebpackSkipAssetsPlugin({
            skipAssets: ["/js/common.js"]
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
