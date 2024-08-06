#include <Windows.h>
#include <algorithm>

#include "attributeMod.hpp"
#include "attributeGetters.cpp"

DWORD WINAPI MainThread(LPVOID lpParam)
{
	std::string section = "attribute mod";
	readConfig(
		std::forward_as_tuple(outOfCombatStamina,section,"enable out of combat stamina loss"_s),
		std::forward_as_tuple(enduranceLightningDefense,section,"enable lightning defense with endurance increase"_s),
		std::forward_as_tuple(hpBonusOnLevel,section,"HP bonus from non-vigor levels"_s),
		std::forward_as_tuple(staminaDamageScaling,section,"scale Damage based on remaining stamina"_s),
		std::forward_as_tuple(minStaminaDamage,section,"damage mult at zero stamina"_s),
		std::forward_as_tuple(maxStaminaDamage,section,"damage mult at full stamina"_s)
	);
	
	from::DLSY::wait_for_system(-1);
	MH_STATUS status = MH_Initialize();
	Log("MinHook Status: ",MH_StatusToString(status));

	hookCall(getCalcCorrectGraph,getCalcCorrectGraphAob,getCalcCorrectGraphOffset,getCalcCorrectGraphSize,&getCalcCorrectGraphOriginal);

	hookCall(getMaxHP,getMaxHPAob,getMaxHPOffset,getMaxHPSize);
	hookCall(getMaxMP,getMaxMPAob,getMaxMPOffset,getMaxMPSize);
	hookCall(getMaxSP,getMaxSPAob,getMaxSPOffset,getMaxSPSize);

	hookCall(calcDamageScale,calcDamageScaleAob,calcDamageScaleOffset,calcDamageScaleSize,&calcDamageScaleOriginal);
	hookFunc(calcSpellScale,calcSpellScaleAob,calcSpellScaleOffset,&calcSpellScaleOriginal);

	hookCall(calcDefense,calcDefenseAob,calcDefenseOffset,calcDefenseSize);
	hookCall(calcResist,calcResistAob,calcResistOffset,calcResistSize);

	hookCall(getMaxEquipLoad,getMaxEquipAob,getMaxEquipOffset,getMaxEquipSize);
	hookCall(getDiscovery,getDiscoveryAob,getDiscoveryOffset,getDiscoverySize);

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