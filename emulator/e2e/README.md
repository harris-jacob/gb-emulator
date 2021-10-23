# e2e
Here is all the source and build bits for the e2e tests for this project. The e2e tests use [blarggs](https://github.com/retrio/gb-test-roms) test roms. We use these roms to test things such as:
- cpu instructions execute as expected
- CPU instructions are timed as expected
- The PPU renders as expected
- We load roms as expected

## Running
These tests will run on PR as part of our CI systems. But if you want to run them locally, you just need to pass the path to the `test roms` this will allow the emulator to load the roms and run the tests correctly.