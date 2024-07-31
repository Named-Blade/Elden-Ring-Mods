.data
	extern returnAddressDamageCalc: qword

.code

	calcDamageOriginal proc
		repeat 15
			nop
		endm
		
		jmp returnAddressDamageCalc
	calcDamageOriginal endp

end