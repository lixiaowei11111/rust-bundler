import { defineConfig } from 'bundler_core';

export default defineConfig({
  entry: "./src/index.js",
  output: {
    path: "./dist",
    filename: "bundle.js",
    chunkFilename: "[name].chunk.js"
  },
  module: {
    rules: [
      {
        test: "\\.js$",
        useLoader: "javascript",
        exclude: "node_modules"
      },
      {
        test: "\\.ts$",
        useLoader: "typescript", 
        exclude: "node_modules"
      },
      {
        test: "\\.json$",
        useLoader: "json"
      }
    ]
  },
  resolve: {
    extensions: [".js", ".ts", ".json"],
    alias: {}
  },
  plugins: [],
  mode: "Development"
});