const path = require('path');
const ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');

module.exports = {
  entry: './ui/index.ts',
  module: {
    rules: [
      {
        test: /\.ts$/,
        exclude: /node_modules/,
        use: 'babel-loader',
      }
    ]
  },
  plugins: [
    new ForkTsCheckerWebpackPlugin(),
  ],
  resolve: {
    extensions: ['.ts', '.js'],
  },
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'ui.dist'),
  },
  devtool: 'source-map',
  devServer: {
    contentBase: './ui.dist',
    watchOptions: {
      ignored: ['node_modules/**', '**/.*.swp'],
    },
    proxy: {
      '/api': 'http://localhost:8000',
    },
  },
};
