#include <Windows.h>
#include <algorithm>

#include "attributeMod.hpp"
#include "attributeGetters.cpp"

DWORD WINAPI MainThread(LPVOID lpParam)
{
	std::string section = "attribute mod";
	readConfig(
		std::forward_as_tuple(outOfCombatStamina,section,"enable out of combat stamina loss"_s)
	);
	
	from::DLSY::wait_for_system(-1);
	MH_STATUS status = MH_Initialize();
	Log("MinHook Status: ",MH_StatusToString(status));

	hookFunc(getCalcCorrectGraphHook,getCalcCorrectGraphAob,getCalcCorrectGraphOffset,getCalcCorrectGraphSize,&getCalcCorrectGraph);

	hookFunc(getMaxHPHook,getMaxHPAob,getMaxHPOffset,getMaxHPSize);
	hookFunc(getMaxMPHook,getMaxMPAob,getMaxMPOffset,getMaxMPSize);
	hookFunc(getMaxMPHook,getMaxSPAob,getMaxSPOffset,getMaxSPSize);

	hookFunc(getMaxEquipLoadHook,getMaxEquipAob,getMaxEquipOffset,getMaxEquipSize);
	hookFunc(getDiscoveryHook,getDiscoveryAob,getDiscoveryOffset,getDiscoverySize);

	MH_STATUS applyStatus = MH_ApplyQueued();
	Log("Apply Status: ",MH_StatusToString(applyStatus));
	
	if (outOfCombatStamina) {
		performPatch(outOfcombatStaminaAob,"0f 94 c2","b2 00 90",outOfcombatStaminaOffset);
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