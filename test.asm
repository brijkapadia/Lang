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
	push rbp
	mov rbp, rsp
	sub rsp, 4
	mov dword [rbp-4], 9
	push rbp
	mov rbp, rsp
	sub rsp, 4
	mov dword [rbp-8], 10
	leave
	leave
	leave 
	ret