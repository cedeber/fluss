/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
    mount: {
        public: { url: "/", static: true },
        src: { url: "/dist" },
    },
    plugins: [
        "@snowpack/plugin-react-refresh",
        "@snowpack/plugin-dotenv",
        "@snowpack/plugin-typescript",
        ["@snowpack/plugin-sass", { style: "compressed" }],
    ],
    packageOptions: {
        polyfillNode: true,
    },
    optimize: {
        // bundle: true,
        splitting: true, // maybe not
        treeshake: true, // maybe not
        minify: true,
        target: "es2018",
    },
};
