# TypeScript + React + Electron template

## Features

- TypeScript + React
- Electron
    - Use [electron-builder](https://github.com/electron-userland/electron-builder) to build the app
- Preload Support
    - Also works on production build
    - Provides context sharing between *main* and *renderer* with the following two types:
        - `APIKey` : String Literal Type
        - `API` : Object Type
    - In renderer process, context objects can be accessed by `window.{APIKey}.{keyof API}`
    - In preload-side, objects can be exposed to the renderer process by calling `contextBridge.exposeInMainWorld({APIKey}, {API})` 
    - Security issues
        - `nodeIntegration` is set to `false`
        - `contextIsolation` is set to `true`

- DevTools Extensions
    - [Redux DevTools](https://chrome.google.com/webstore/detail/redux-devtools/lmhkpmbekcpmknklioeibfkpmmfibljd)
    - [React Developer Tools](https://chrome.google.com/webstore/detail/react-developer-tools/fmkadmapgofadopljbjfkapdkoienihi)

**note**: This template uses DevTools Extensions installed in the host's Google Chrome
- It has been confirmed to work in the following environments:
        - [ ] Windows
        - [ ] Mac
        - [x] Linux

## Scripts

- `yarn dev`
    - Launch a development server and an electron window
    - Quit by *Ctrl-C*

- `yarn build`
    - Pack the app into `dist` directory as an installer

## Installation
1. Clone this repository
2. Run `yarn install`
