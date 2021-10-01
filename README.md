# gb-emulator
Game Boy emulator written in C. 

## Technologies
- [CMAKE](https://cmake.org/)
- [Docker](https://www.docker.com/)
- [vscode](https://code.visualstudio.com/)
- [Remote Containers](https://code.visualstudio.com/docs/remote/containers)
- [Unity](http://www.throwtheswitch.org/unity)

## setup/build
If you want to build/test/work with this repo's source, it is recommended that you run it with the provided devcontainer. To do this you will need [docker](https://www.docker.com/), [vscode](https://code.visualstudio.com/) and the [remote containers extension](https://code.visualstudio.com/docs/remote/containers) installed.
Once you've done this, you can follow these steps:

- run a  `git submodule update --init` to clone with submodules.
- open repo directory in vscode
- select the `remote containers: build and open in container` option from the VScode context menu (`F1` to access).
- once the container has built, you can run a CMAKE configure from the VScode context menu (the devcontainer should install the cmake plugin for you).
- Select the target you'd like to build from the bottom of vscode [test] or [production]. Use  `F7` to build.


## progress
- [x] repo setup 
- [x] cmake setup
- [x] Unity setup
- [x] MMU implementation
- [x] Register implementation
- [x] All ALU/BIN operations
- [x] CPU implmentation
- [ ] PPU implementation
- [ ] UI 
- [ ] Web build?
- [ ] 
