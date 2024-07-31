.data
extern return_addresses: qword
return_address qword 1
extern jump_address: qword
extern jump_address_uncapped_level: qword
stat_cap dword 1
extern getStatCap: qword

.code
	
	call_before Macro
		push rax
		push rcx
		push rdx
		push r8
		push r9
		push r10
		push r11
	endm
	
	call_after Macro
		pop r11
		pop r10
		pop r9
		pop r8
		pop rdx
		pop rcx
		pop rax
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
		mov ecx,0
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