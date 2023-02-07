.global main

.equ N, 8

.data
A: .word 7,3,25,4,75,2,1,1

.bss
B: .space 4*N

.text

MaxVector:
    add sp, sp, -16
    sw s1, 0(sp)

    mv s1, zero
    mv t2, zero
    loop2:
        beq s1, a1, endloop2
        lw t1, (a0)
        ble t1, t2, else2
          mv t2, t1
          mv t3, a0
        else2:
        add a0, a0, 4
        add s1, s1, 1
        j loop2
    endloop2:
    sw zero, (t3)

    mv a0, t2
    lw s1, 0(sp)
    add sp, sp, 16
    ret


SortVector:
    add sp, sp, -32
    sw s1, 0(sp)
    sw s2, 4(sp)
    sw s3, 8(sp)
    sw ra, 12(sp)

    mv s1, a0               # Address of vector A
    mv s2, a1               # Address of vector B
    mv s3, a2               # Size of vectors A and B
    mv t1, zero

    loop1:
        beq t1, s3, endloop1
        mv a0, s1
        mv a1, s3
        sw t1, 16(sp)
        jal MaxVector
        lw t1, 16(sp)
        sw a0, (s2)
        add s2, s2, 4
        add t1, t1, 1
        j loop1
    endloop1:

    lw s1, 0(sp)
    lw s2, 4(sp)
    lw s3, 8(sp)
    lw ra, 12(sp)
    add sp, sp, 32
    ret


main:
    add sp, sp, -16
    sw ra, 0(sp)

    la a0, A
    la a1, B
    add a2, zero, N
    jal SortVector

    lw ra, 0(sp)
    add sp, sp, 16
    ret


.end