#include <Windows.h>
#include <algorithm>

#include "rallyMod.hpp"

void rallyHitHook(HookContext* context){
	RallyData& rallyData = *(RallyData*)((context->rbx)+0x160);
	float& rallyAmount = context->imm0[0];
	
	if (rallyAmount > 0.0 && rallyResetOnHit){
		
		if (rallyData.rallyTimer < rallyConfigTime){
			rallyData.rallyTimer = rallyConfigTime;
		}
		rallyData.rallyCap = rallyData.rallyPotential;
	}
	
}

void rallyChangeHook(HookContext* context){
	int& health = *(int*)((context->rbx)+0x138);
	RallyData& rallyData = *(RallyData*)((context->rbx)+0x160);
	uint64_t& startingHealth = (context->rbp);
	int rallyChange = -(health - startingHealth);
	
	if (rallyChange < 0){
		if (rallyKeepOnHeal){
			startingHealth = health;
		}
		if (rallyResetOnHeal){
			if (rallyData.rallyTimer < rallyConfigTime){
				rallyData.rallyTimer = rallyConfigTime;
			}
			rallyData.rallyCap = rallyData.rallyPotential;
		}
	}
}



DWORD WINAPI MainThread(LPVOID lpParam)
{
	std::string section = "rally mod";
	readConfig(
		std::forward_as_tuple(rallyConfigTime,section,"default rally timer"_s),
		std::forward_as_tuple(rallyConfigDecay,section,"rally decay"_s),
		std::forward_as_tuple(rallyResetOnHit,section,"reset rally time on hit"_s),
		std::forward_as_tuple(rallyResetOnHeal,section,"reset rally time on heal"_s),
		std::forward_as_tuple(rallyKeepOnHeal,section,"keep rally on heal"_s)
	);
	from::DLSY::wait_for_system(-1);
	if (!CallHook::initialize()) return 0;
	
	uintptr_t rallyTimeAddress = AobScan(rallyTimeAob);
	if (rallyTimeAddress != 0){
		rallyTime = reinterpret_cast<float *>(rallyTimeAddress + (*(std::uint32_t*)(rallyTimeAddress+rallyTimeOffset)) + (rallyTimeSize + rallyTimeOffset));
	}
	
	uintptr_t rallyDecayAddress = AobScan(rallyDecayAob);
	if (rallyDecayAddress != 0){
		rallyDecay = reinterpret_cast<float *>(rallyDecayAddress + (*(std::uint32_t*)(rallyDecayAddress+rallyDecayOffset)) + (rallyDecaySize + rallyDecayOffset));
	}
	
	uintptr_t rallyHitAddress = AobScan(rallyHitAob);
	if (rallyHitAddress != 0){
		auto hook1 = new CallHookTemplate<ContextHookV>(reinterpret_cast<void *>(rallyHitAddress), rallyHitHook);
	}
	
	uintptr_t rallyChangeAddress = AobScan(rallyChangeAob);
	if (rallyChangeAddress != 0){
		auto hook2 = new CallHookTemplate<ContextHook>(reinterpret_cast<void *>(rallyChangeAddress), rallyChangeHook);
	}
	
	
	if (rallyTime != nullptr){
		*rallyTime = rallyConfigTime;
	}
	if (rallyDecay != nullptr){
		*rallyDecay = 1.0/rallyConfigDecay;
	}
	
	return 0;
}

BOOL WINAPI DllMain(HINSTANCE module, DWORD reason, LPVOID)
{
	if (reason == DLL_PROCESS_ATTACH)
	{
		DisableThreadLibraryCalls(module);
		CreateThread(0, 0, &MainThread, 0, 0, NULL);
	}
	return 1;
}