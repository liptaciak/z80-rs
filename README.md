# zin
Z80 Emulator written in Rust :crab:


First, generate binary file using Assembler. For example:

Using [Sjasmplus](https://github.com/z00m128/sjasmplus):


Generate binary file:
`sjasmplus --raw=z80.bin z80.asm`


and run the emulator using:

`zin z80.bin`
