global _start
section .text

_start:
    call main
    call exit

exit:
    mov rax, 60
    mov rdi, 0
    syscall

main:
    push rbp
    mov rbp, rsp
	sub rsp, 4
	mov dword [rbp-4], 0
