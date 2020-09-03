const path = require('path')
const VueLoaderPlugin = require('vue-loader/lib/plugin')
const FileManagerPlugin = require('filemanager-webpack-plugin')


module.exports = {
  entry: './src/index.js',
  output: {
    path: path.resolve(__dirname, './dest'),
    filename: 'bundle.js'
  },
  devServer: {
    contentBase: path.resolve(__dirname, 'public')
  },
  module: {
    rules: [
      {
        test: /\.vue$/,
        loader: 'vue-loader'
      },
      {
        test: /\.js$/,
        loader: 'babel-loader',
      },
      {
        test: /\.css$/,
        use: ['vue-style-loader', 'css-loader']
      },
    ]
  },
  resolve: {
    extensions: ['.js', '.vue'],
    alias: {
      vue$: 'vue/dist/vue.esm.js',
    },
  },
  plugins: [
    new VueLoaderPlugin(),
    new FileManagerPlugin({
      onEnd: {
        copy: [
          {
            source: path.resolve(__dirname, 'dest', 'bundle.js'),
            destination: path.resolve(__dirname, '../', 'src', 'bundle.js'),
          }
        ]
      }
    })
  ],
}
