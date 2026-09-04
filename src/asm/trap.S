# assembly-level trap handler
.section .text 
.global asm_trap_vector
asm_trap_vector:
		# we get here when cpu is interrupted
		# for any reason 
		mret

