.data
extern return_addresses: qword
return_address qword 1
extern jump_address: qword
extern jump_address_uncapped_level: qword
stat_cap dword 1
extern getStatCap: qword

.code
	
	call_before Macro
		pushfq
		push rax
		push rcx
		push rdx
		push r8
		push r9
		push r10
		push r11
		push r15

		mov r15, rsp
		and rsp, -16

		sub rsp, 32 * 16
		movaps [rsp + 32 * 15], xmm15
		movaps [rsp + 32 * 14], xmm14
		movaps [rsp + 32 * 13], xmm13
		movaps [rsp + 32 * 12], xmm12
		movaps [rsp + 32 * 11], xmm11
		movaps [rsp + 32 * 10], xmm10
		movaps [rsp + 32 * 9], xmm9
		movaps [rsp + 32 * 8], xmm8
		movaps [rsp + 32 * 7], xmm7
		movaps [rsp + 32 * 6], xmm6
		movaps [rsp + 32 * 5], xmm5
		movaps [rsp + 32 * 4], xmm4
		movaps [rsp + 32 * 3], xmm3
		movaps [rsp + 32 * 2], xmm2
		movaps [rsp + 32 * 1], xmm1
		movaps [rsp + 32 * 0], xmm0
	endm
	
	call_after Macro
		movaps xmm0, [rsp + 32 * 0]
		movaps xmm1, [rsp + 32 * 1]
		movaps xmm2, [rsp + 32 * 2]
		movaps xmm3, [rsp + 32 * 3]
		movaps xmm4, [rsp + 32 * 4]
		movaps xmm5, [rsp + 32 * 5]
		movaps xmm6, [rsp + 32 * 6]
		movaps xmm7, [rsp + 32 * 7]
		movaps xmm8, [rsp + 32 * 8]
		movaps xmm9, [rsp + 32 * 9]
		movaps xmm10, [rsp + 32 * 10]
		movaps xmm11, [rsp + 32 * 11]
		movaps xmm12, [rsp + 32 * 12]
		movaps xmm13, [rsp + 32 * 13]
		movaps xmm14, [rsp + 32 * 14]
		movaps xmm15, [rsp + 32 * 15]

		mov rsp,r15

		pop r15
		pop r11
		pop r10
		pop r9
		pop r8
		pop rdx
		pop rcx
		pop rax
		popfq
	endm
	
	get_address proc
		push rax
		mov rax, OFFSET return_addresses
		lea rax,[rax+rcx*8]
		mov rax,[rax]
		mov return_address,rax
		pop rax
		ret
	get_address endp
	
	compare_levels proc
	
		call_before
		mov rax,OFFSET getStatCap
		mov ecx,[r15+200];this is cursed
		call rax
		mov stat_cap,eax
		call_after
	
		cmp ebx,stat_cap
		jg stat_capped
		
		jmp after
		self_locator proc
			mov rax,$+11
			ret
		self_locator endp
		after:
		
		repeat 11
			nop
		endm
		
		push rcx
		mov rcx,0
		call get_address
		pop rcx
		jmp return_address
		
		stat_capped:
			jmp jump_address
	compare_levels endp
	
	levelCostOriginal proc
	
		repeat 18
			nop
		endm
	
		push rcx
		mov rcx,1
		call get_address
		pop rcx
		jmp return_address
		
	levelCostOriginal endp
	
end