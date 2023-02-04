# Wikit App

安装 quasar

    npm install -g @quasar/cli
    
    or
    
    yarn global add @quasar/cli

创建工程

    quasar create <project>
    
    or
    
    yarn create quasar

安装 tauri cli

    npm install -D @tauri-apps/cli
    
    or
    
    yarn global add @tauri-apps/cli
  

在 package.json 的 scripts 中加入如下选项

    "scripts": {
      "tauri": "tauri",
    },

然后安装 tauri api

    npm install @tauri-apps/api
    
    or
    
    yarn global add @tauri-apps/api

初始化 tauri

    npm run tauri init
    
    or
    
    yarn tauri init

开发与构建

    quasar dev
    quasar build
    
    or 
    
    yarn quasar dev
    

