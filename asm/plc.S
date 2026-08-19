.intel_syntax noprefix

.section .data

.align 4
plc_memory:
    .zero 4096

.section .text

.global plc_init
.global plc_read_u32
.global plc_write_u32
.global plc_shutdown

.type plc_init, @function
plc_init:
    xor eax, eax
    ret


.type plc_read_u32, @function
plc_read_u32:
    /*
     * System V x86-64 ABI:
     *
     * RDI = address
     * RSI = pointer to output value
     *
     * Return:
     *   EAX = 0 success
     *   EAX = -1 invalid address
     *   EAX = -2 null pointer
     */

    test rsi, rsi
    jz .read_null

    cmp edi, 4092
    ja .read_invalid

    /*
     * Address must be 32-bit aligned.
     */
    test edi, 3
    jnz .read_invalid

    lea rcx, plc_memory
    mov eax, edi
    add rcx, rax

    mov eax, DWORD PTR [rcx]
    mov DWORD PTR [rsi], eax

    xor eax, eax
    ret

.read_null:
    mov eax, -2
    ret

.read_invalid:
    mov eax, -1
    ret


.type plc_write_u32, @function
plc_write_u32:
    /*
     * RDI = address
     * RSI = value
     *
     * Return:
     *   EAX = 0 success
     *   EAX = -1 invalid address
     */

    cmp edi, 4092
    ja .write_invalid

    test edi, 3
    jnz .write_invalid

    lea rcx, plc_memory
    mov eax, edi
    add rcx, rax

    mov DWORD PTR [rcx], esi

    xor eax, eax
    ret

.write_invalid:
    mov eax, -1
    ret


.type plc_shutdown, @function
plc_shutdown:
    xor eax, eax
    ret
