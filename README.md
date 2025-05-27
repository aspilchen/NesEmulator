# NESEmulator
NES Emulator in Rust. This is a work in progress.. Currently doing a major refactor to clean up the file structure, as well as implementing the cartridge/ROM to allow for the use of 3rd party test ROMs.

## Details/Current Progress

### Opcodes/CPU Instructions
CPU Instructions currently exist as functions that operate on the Bus. This might change in the future as timing system timing needs to be implemented.

### Memory Mapping
The NES relies heavily on memory mapping. My approach is to treat the system as a tree, where the bus is the root. A read/write to the bus will be mapped to the appropriate device, which will contain its own internal memory mapping procedures.

## TODO:
- Complete CPU instructions (validation, etc.)
- Graphics
- Audio
- System clock and timing