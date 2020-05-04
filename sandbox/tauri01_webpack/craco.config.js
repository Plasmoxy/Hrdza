const merge = require('webpack-merge')
const tauriCfg = require('@tauri-apps/tauri-webpack').config()

module.exports = {
  webpack: {
    configure: (cfg, {env, paths}) => {
      console.log("Craco intercepted webpack config with Tauri cfg")
      return merge(cfg, tauriCfg);
    }
  }
};