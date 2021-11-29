module.exports = {
    publicPath: '',
    css: {
        loaderOptions: {
            scss: {
                additionalData: `@import "@/assets/sass/main.scss";` //@はsrcと同義
            }
        }
    },
    devServer: {
        proxy: 'http://127.0.0.1:8081',
    }
}
  