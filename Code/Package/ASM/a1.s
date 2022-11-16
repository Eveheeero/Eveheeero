	.text
	.global _Z2a1v
_Z2a1v:
	pushq	%rbp
	movq	%rsp,	%rbp
	movl	$1,	%eax
	popq	%rbp
	ret
a1Origin:
	push	%rbp
	mov	%rsp,	%rbp
	mov	$1,	%rax
	pop	%rbp
	ret
