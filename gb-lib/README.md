## CPU
CPU logic of my Emulator. Deals with handling assembly instructions and memory management.

### Resources
Useful resources I used for writing the CPU:
- Full tech spec: http://bgb.bircd.org/pandocs.htm
- For memory structure: http://gameboy.mongenel.com/dmg/asmmemmap.html
- For MMU implementation: https://www.linkedin.com/pulse/creating-gameboy-emulator-part-1-bruno-croci/
- CPU hardware details and registers: https://medium.com/@raphaelstaebler/building-a-gameboy-from-scratch-part-2-the-cpu-d6986a5c6c74


### CPU Registers
The CPU has 6x16bit registers (actually a bunch of 8 bit registers that can be accessed in pairs)

| Register     | Description                                                                |
| -----------  | ---------------------------------------------------------------------------|
| AF           | two 8 bit registers. A stands for accumulator (does arithmetic operations) F is the flag register.|
| BC && DE      | Nothing special just 2 8 bit registers.|
| HL           | Can be used as 2x8 bit. Can also be used as 16bit register (can be used to point to memory address) |
| SP           | Stack pointer, 16bits and points to stack |
| PC           | Program counter, pointer to the next instruction in memory.

interesting points to note are the CPU itself is 8 bit, but memory addresses can be accessed in pairs. Additionally, the program counter and stack pointer are 18 bits. This means we have 2^16 addressable space.

### Memory
The CPU interacts with the outside world through the RAM. Accessible memory is kept in the Address Bus (16bit means there are 2^16 available adresses.). The program counter (PC) loads and holds an instruction from memory and the stack pointer (SP) points to a location in memory.

