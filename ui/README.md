# ui

This folder houses the react app that loads and runs the emulator's wasm module & houses the debugger I am using to fix issues in the emulator code.

## public/

All static resources we want to serve and access from the browser at runtime go here. This is where the wasm output and rom data files get put when they are generated.

## src/

The source code is split as follows:

- The app entry point is `index.tsx` this is where reactdom plops our UI onto the dom.
- All generic styling data is kept in `theme.ts`.

### components/

- All react components live here
- Anything under the `debugger` is debugger specific, see below.
- Anything under `generic` is a basic HTML element wrapper.

### emulator/

The code here controls the execution of the emulator WASM file, it contains typings for the js bindings which execute the exposed C functions in the emulator. It also contains some abstractions and wrappers designed to make working with the emulator easier. In the case of the debugger, it does some pretty horrid stuff with the WASM heap, proceed with caution.

### hooks/

The hooks here are mainly utilities for operating with the emulator from within the context of the react lifecyle. We want to use one "emulator" instance and be able to operate it from multiple places, so a context is used (I have my doubts that this is a good thing).

### routes/

Routes contain top level pages, mainly the debugger for now (as thats all I have currently).

## The Debugger

The debugger is designed for me to step through opcodes, visualize the state of the CPU/memory, and generally debug things. It is therefore, not designed very nicely. React does lots of the heavy lifting and we really just call the C functions directly on a single thread and then use our knowledge of the WASM heap to see what's happening in memory. Any multithreading through webworkers (for example to run the CPU until we hit a breakpoint) is handled by React's effect hook, this will not be the case for the actual emulator. I am hoping to leverage emscripten's PThread support so that we can directly use threads in the C code and simply present state to the browser and have it signal control inputs back, but I have no idea how to do this yet.
