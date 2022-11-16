int test(int a, long b, long long c, char d, char e, char f, long long h)
{
    return 1;
}
/*

        .file   "test.cpp"
        .text
        .globl  _Z4testv
        .type   _Z4testv, @function
_Z4testv:
.LFB0:
        .cfi_startproc
        endbr64
        pushq   %rbp
        .cfi_def_cfa_offset 16
        .cfi_offset 6, -16
        movq    %rsp, %rbp
        .cfi_def_cfa_register 6
        movl    $1, %eax
        popq    %rbp
        .cfi_def_cfa 7, 8
        ret
        .cfi_endproc
.LFE0:
        .size   _Z4testv, .-_Z4testv
        .ident  "GCC: (Ubuntu 9.4.0-1ubuntu1~20.04.1) 9.4.0"
        .section        .note.GNU-stack,"",@progbits
        .section        .note.gnu.property,"a"
        .align 8
        .long    1f - 0f
        .long    4f - 1f
        .long    5
0:
        .string  "GNU"
1:
        .align 8
        .long    0xc0000002
        .long    3f - 2f
2:
        .long    0x3
3:
        .align 8
4:
*/

/*
        movl    %edi, -4(%rbp)  int
        movq    %rsi, -16(%rbp) long
        movq    %rdx, -24(%rbp) long long
        movl    %ecx, %eax      char edi rsi rdx rcx 순으로 인자가 들어오는듯?
        movb    %al, -8(%rbp)
*/
/*int a, long b, long long c, char d, char e, char f, long long h 가 인자일때
        movl    %edi, -4(%rbp)  1
        movq    %rsi, -16(%rbp) 2
        movq    %rdx, -24(%rbp) 3
        movl    %ecx, %eax      4
        movl    %r8d, %ecx      5
        movl    %r9d, %edx      6
        movb    %al, -8(%rbp)
        movl    %ecx, %eax
        movb    %al, -28(%rbp)
        movl    %edx, %eax
        movb    %al, -32(%rbp)
*/
