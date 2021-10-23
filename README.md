# gb-emulator
Game Boy emulator written in C and compiled to WASM for consumption by a preact app. 
## Technologies
- [CMAKE](https://cmake.org/)
- [Docker](https://www.docker.com/)
- [vscode](https://code.visualstudio.com/)
- [Remote Containers](https://code.visualstudio.com/docs/remote/containers)
- [Unity](http://www.throwtheswitch.org/unity)

## Structure
All emulator specific code (C) is housed inside the `emulator` folder. The provided scripts can be used to compile these files to wasm ready for the browser.
The UI (a preact-app) is stored inside the `ui` folder.

## setup/build
If you want to build/test/work with this repo's source, it is recommended that you run it with the provided devcontainer. To do this you will need [docker](https://www.docker.com/), [vscode](https://code.visualstudio.com/) and the [remote containers extension](https://code.visualstudio.com/docs/remote/containers) installed.
Once you've done this, you can follow these steps:

- run a  `git submodule update --init` to clone with submodules.
- open repo directory in vscode
- select the `remote containers: build and open in container` option from the VScode context menu (`F1` to access).
- once the container has built, you can run a CMAKE configure from the VScode context menu (the devcontainer should install the cmake plugin for you).
- Select the target you'd like to build from the bottom of vscode [test] or [production]. Use  `F7` to build.