#include <Windows.h>
#include <algorithm>

#include "attributeMod.hpp"
#include "attributeGetters.cpp"

void performPatch(const std::string& aob,
	const std::string& expectedBytes,
	const std::string& newBytes,
	size_t offset)
{
	uintptr_t patchAddress = AobScan(aob);

	if (patchAddress != 0)
	{
		patchAddress += offset;

		ReplaceExpectedBytesAtAddress(patchAddress, expectedBytes, newBytes);
	}
}

DWORD WINAPI MainThread(LPVOID lpParam)
{
	std::string section = "attribute mod";
	readConfig(
		std::forward_as_tuple(outOfCombatStamina,section,"enable out of combat stamina loss"_s)
	);
	from::DLSY::wait_for_system(-1);
	MH_Initialize();

	{
		uintptr_t address = AobScan(getMaxHPByStatsAob);
		void* funcAddress = (void*)getAddressFromMemory(address,getMaxHPByStatsOffset,getMaxHPByStatsSize);
		auto hook1 = MH_CreateHook(funcAddress, getMaxHPByStatsHook, nullptr);
		MH_QueueEnableHook(funcAddress);
	}
	{
		uintptr_t address = AobScan(getMaxFPByStatsAob);
		void* funcAddress = (void*)getAddressFromMemory(address,getMaxFPByStatsOffset,getMaxFPByStatsSize);
		auto hook1 = MH_CreateHook(funcAddress, getMaxFPByStatsHook, nullptr);
		MH_QueueEnableHook(funcAddress);
	}
	{
		uintptr_t address = AobScan(getCalcCorrectGraphAob);
		void* funcAddress = (void*)getAddressFromMemory(address,getCalcCorrectGraphOffset,getCalcCorrectGraphSize);
		auto hook1 = MH_CreateHook(funcAddress, getCalcCorrectGraphHook, (void**)&getCalcCorrectGraph);
		MH_QueueEnableHook(funcAddress);
	}

	MH_ApplyQueued();

	
	if (outOfCombatStamina) {
		performPatch(outOfcombatStaminaAob,"0f 94 c2","b2 00 90",outOfcombatStaminaOffset);
	}
	
	Sleep(INFINITE);
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