#pragma once

#include <cmath> 
#include <dantelion2/system.hpp>

using namespace ModUtils;

using calcFlatDefenseOriginalType = float(*)(float,float);

extern "C"
{
	float calcDamageOriginal(
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
	);
	calcFlatDefenseOriginalType calcFlatDefenseOriginal;
	uintptr_t returnAddressDamageCalc;
}

bool healAbsorb = true;
bool flatterDefenses = true;
bool splitDamageFix = true;
float chipDamage = 1.0;
bool heavenOrHell = false;

enum DamageType {physical,magic,fire,lightning,holy};
DamageType damageTypes[5] = {DamageType::physical,DamageType::magic,DamageType::fire,DamageType::lightning,DamageType::holy};