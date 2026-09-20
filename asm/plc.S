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
    test rsi, rsi
    jz .read_null
    cmp edi, 4092
    ja .read_invalid
    test edi, 3 
    jnz .read_invalid

    mov eax, edi
    mov ecx, DWORD PTR plc_memory[rax]
    mov DWORD PTR [rsi], ecx
    
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

    cmp edi, 4092
    ja .write_invalid
    test edi, 3
    jnz .write_invalid
    
    mov eax, edi
    mov DWORD PTR plc_memory[rax], esi
    
    xor eax, eax # 0 success
    ret

.write_invalid:
    mov eax, -1 # 1 invalide addr
    ret

.type plc_shutdown, @function
plc_shutdown:
    xor eax, eax
    ret
