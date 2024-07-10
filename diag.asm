; Z80 CPU DIAGNOSTIC TEST

CPUTEST:
    LD SP, 0x07AD ; Set the stack pointer
    AND 0 ; Initalize A register
    JP Z, J010 ; Test JZ
    CALL CPUFAIL

J010:
    JP NC, J020 ; Test JNC
    CALL CPUFAIL

J020:
    JP PE, J030 ;Test JPE
    CALL CPUFAIL

J030:
    JP NZ, J050 ; Test JNZ
    JP C, J050 ; Test JC
    JP PO, J050 ; Test JPO
    JP M, J050 ; Test JM
    JP J060

J050: CALL CPUFAIL

J060:
    ADD A, 6 ; A = 6
    JP NZ, J070 ; Test JNZ
    CALL CPUFAIL

J070:
    JP C, J080 ; Test JC
    JP J090

J080: CALL CPUFAIL

J090:
    ADD A, 0x70 ; A = 0x76
    JP PO, J100 ; Test JPO
    CALL CPUFAIL

J100:
    JP M, J110 ; Test JM
    JP Z, J110 ; Test JZ
    JP NC, J120 ; Test JNC

J110: CALL CPUFAIL

J120:
    ADD A, 0x81 ; A = 0xF7
    JP M, J130 ; Test JM
    CALL CPUFAIL

J130:
    JP Z, J140 ; Test JZ
    JP C, J140 ; Test JC
    JP J150

J140: CALL CPUFAIL

J150:
    ADD A, 0xFE ; A = 0xF5
    JP C, J160 ; Test JC
    CALL CPUFAIL

J160:
    JP Z, J170 ; Test JZ
    JP P, J170 ; Test JP

    CALL CPUOK

J170: CALL CPUFAIL

CPUOK: ; Call when CPU passes
    HALT
    JP CPUOK

CPUFAIL: ; Call when CPU fails
    HALT
    JP CPUFAIL