	/* assemble with
	   riscv-none-embed-as -march=rv32imac -o p2jit.o p2jit.s
	*/

	.section .jitinterp, "a"
	.globl _interpstart
_interpstart:
	.incbin "p2lut.bin"

	.text
	.globl _riscv_start
_riscv_start:
	
