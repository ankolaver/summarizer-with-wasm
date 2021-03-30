const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  //main file to start from
  entry: "./bootstrap.js",
  //output of the bundled javascript
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  //operate in development mode
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
};
