# gb-emulator
A WIP gameboy emulator written in C and compiled to WASM for running in a browser.

# Goal
The goal of this project was mainly to learn the inner workings of the gameboy and, more specifically, its CPU. 

## Structure
All emulator specific code (C) is housed inside the `emulator` folder. The provided scripts can be used to compile these files to wasm ready for the browser.
The UI (a react-app) is stored inside the `ui` folder and can built as a single page application for consumption by the browser. To read more about the repo structure and design choices, see the project sub-directories.

## Getting started
Instructions for how to get up and running with the code base:

### dev environment
If you want to build/test/work with this repo's source, it is recommended that you run it with the provided devcontainer. To do this you will need [docker](https://www.docker.com/), [vscode](https://code.visualstudio.com/) and the [remote containers extension](https://code.visualstudio.com/docs/remote/containers) installed.
Once you've done this, you can follow these steps:

- run a  `git submodule update --init` to clone with submodules.
- open repo directory in vscode
- select the `remote containers: build and open in container` option from the VScode context menu (`F1` to access).

## Setup
To setup the project, you will need to run a `yarn` at the project root. This will install the necessary javascript libraries to run the frontend. It will also compile the emulator and generate the wasm and supporting javascript used to load and execute it.

### running
To run the react development server, use:
```
yarn dev
```
To compile and build the production react app run:
```
yarn build
```
To recompile emulator and regenerate the wasm buildings run:
```
yarn build-emulator
```
> TODO: get react server to recompile emulator when source files are updated ?

### Testing
To keep myself sane, I wrote a bunch of tests for checking CPU opcodes (there are a lot to write and they're easy to get wrong). To build and run these tests:
```
yarn test-emulator
```