const merge = require('webpack-merge')
const tauriCfg = require('@tauri-apps/tauri-webpack').config()

module.exports = function override(config, env) {
  console.log("Webpack was rewired." + env.TAURI)
  return merge(tauriCfg, config);
}