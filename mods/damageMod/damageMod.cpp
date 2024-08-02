#include <Windows.h>

#include "damageMod.hpp"

float handleChipDamage(float value) {
	float sign =  std::signbit(value) ? -1.0f : 1.0f;
	value = std::abs(value);
    if (value > 0){
		if (value < chipDamageTotal){
			return chipDamageTotal * sign;
		} else {
			return value * sign;
		}
	} else {
		return 0;
	}
}

float calcFlatDefense(float damage,float defense)
{
	float finalDamage = damage-defense;
	
	if (finalDamage < chipDamageFlat){
		if (damage >= chipDamageFlat) {
			return chipDamageFlat;
		} else {
			return damage;
		}
	} else {
		return finalDamage;
	}
}

float calcDamage(
		float returnArr[],
		float damageArr[],
		float flatDefenseArr[],
		float multArr_1[],
		float multArr_2[],
		float damageNegationArr[],
		float multArr_4[],
		float multArr_5[],
		float multArr_6[],
		float multArr_7[],
		float multArr_8[],
		float maxHealth,
		bool _2,
		bool _3,
		float _4,
		float damageMult
	)
{
	
	float totalDamage = 0.0;
	float totalDamageRaw = 0.0;
	float totalDefense = 0.0;
	
	for (DamageType damageType : damageTypes)
	{
		totalDamageRaw += damageArr[damageType];
	}
	
	if (heavenOrHell && totalDamageRaw > 0.0)
	{
		return maxHealth;
	}
	
	for (DamageType damageType : damageTypes)
	{
		float damagePortion = damageArr[damageType]/totalDamageRaw;
		totalDefense += damagePortion*flatDefenseArr[damageType];
		
		float flatDefense;
		float damageBase;
		if (splitDamageFix && flatterDefenses) {
			flatDefense = damagePortion*flatDefenseArr[damageType];
			damageBase = calcFlatDefense(damageArr[damageType],flatDefense);
		} else if (flatterDefenses) {
			damageBase = calcFlatDefense(damageArr[damageType],flatDefenseArr[damageType]);
		} else if (!splitDamageFix){
			damageBase = calcFlatDefenseOriginal(damageArr[damageType],flatDefenseArr[damageType]);
		} else {
			damageBase = damageArr[damageType];
		}
		
		float damage = damageBase*
			multArr_1[damageType]*
			multArr_2[damageType]*
			damageNegationArr[damageType]*
			multArr_4[damageType]*
			multArr_5[damageType]*
			multArr_6[damageType]*
			multArr_7[damageType]*
			multArr_8[damageType];
		
		totalDamage += damage;
		returnArr[damageType] = damage*damageMult;
	}
	
	if (splitDamageFix && !flatterDefenses)
	{
		totalDamage = calcFlatDefenseOriginal(totalDamage,totalDefense);
		float damageFrac = totalDamage/totalDamageRaw;
		for (DamageType damageType : damageTypes){
			returnArr[damageType] = returnArr[damageType]*damageFrac;
		}
	}
	
	float finalDamage = totalDamage*damageMult;
	
	if (!healAbsorb && finalDamage < 0.0)
	{
		finalDamage = 0.0;
	}
	
	//stupidity guard
	if (std::isnan(finalDamage)){
		return 0.0;
	}
	
	if (chipDamageTotal > 0){
		finalDamage = handleChipDamage(finalDamage);
	}
	return finalDamage;
}


DWORD WINAPI MainThread(LPVOID lpParam)
{
	Log("Starting damage calc mod");
	
	std::string section = "damage mod";
	readConfig(
		std::forward_as_tuple(healAbsorb,section,"healing absorption"_s),
		std::forward_as_tuple(flatterDefenses,section,"flatter defenses"_s),
		std::forward_as_tuple(chipDamageFlat,section,"chip damage (flat)"_s),
		std::forward_as_tuple(chipDamageTotal,section,"chip damage (total)"_s),
		std::forward_as_tuple(splitDamageFix,section,"split damage fix"_s),
		std::forward_as_tuple(heavenOrHell,section,"Heaven or Hell"_s)
	);
	
	from::DLSY::wait_for_system(-1);
	
	{// hook damage calculation
		std::string aob = "48 8b c4 48 89 58 18 4c 89 48 20 48 89 50 10 48 89 48 08 55 56 57 41 54 41 55 41 56 41 57 48 81 ec a0 00 00 00";
		uintptr_t patchAddress = AobScan(aob);
		
		if (patchAddress != 0){
			
			uintptr_t func = reinterpret_cast<uintptr_t>(&calcDamage);
			uintptr_t orginalFunc = reinterpret_cast<uintptr_t>(&calcDamageOriginal);
			
			returnAddressDamageCalc = patchAddress+15;
			MemCopy(orginalFunc, patchAddress, 15);
			Hook(patchAddress, func, 1);
		}
	}
	{//get flat defense calculation
		std::string aob = "48 83 ec 48 f3 0f 10 15 ?? ?? ?? ?? 0f 28 e0 f3 0f 11 54 24 50 0f 57 c0 81 64 24 50 ff ff ff 7f 0f 28 d0 f3 0f 10 5c 24 50 f3 0f 5c d3";
		uintptr_t funcAddress = AobScan(aob);
		
		if (funcAddress != 0)
		{
			calcFlatDefenseOriginal = reinterpret_cast<calcFlatDefenseOriginalType>(funcAddress);
		}
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