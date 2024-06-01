# z80
Z80 Emulator written in Rust :crab:


First, generate binary file using Assembler. For example:

Using [Sjasmplus](https://github.com/z00m128/sjasmplus):

#### **`z80.asm`**
```asm
start: 
    halt
    jp start
    
    end start
```

this code will create infinite loop.


Generate binary file:
`sjasmplus --raw=z80.bin z80.asm`


and run the emulator using:

`cargo run z80.bin`
