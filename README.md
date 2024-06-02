# z80
Z80 Emulator written in Rust :crab:


First, generate binary file using Assembler. For example:

Using [Sjasmplus](https://github.com/z00m128/sjasmplus):

#### **`z80.asm`**
```asm
start:
    ld a, 5
    
loop:
    cp b
    jr z, end

    inc b
    jp loop
    
end:
    halt 
    jp end
    
    end start
```

this code will create a loop that executes 5 times.


Generate binary file:
`sjasmplus --raw=z80.bin z80.asm`


and run the emulator using:

`z80 z80.bin`
