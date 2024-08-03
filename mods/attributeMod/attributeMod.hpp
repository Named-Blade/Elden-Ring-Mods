#pragma once
#include <iostream>

#include <dantelion2/system.hpp>
#include <param/param.hpp>
#include <MinHook.h>

#include <ModUtils.hpp>
#include <config.hpp>

using namespace ModUtils;

std::string getMaxHPByStats = "41 89 4e 08 8b 4e 1c 41 89 4e 10 48 8b ce e8 ?? ?? ?? ?? 8b c8 49 8b d4 e8 ?? ?? ?? ?? f3 0f 2c c8 41 89 4e 0c 48 8b ce";
int getMaxHPByStatsOffset = 15;
int getMaxHPByStatsSize = getMaxHPByStatsOffset + 4;

int attributeDataOffset = 0x288;
struct AttributeData{
    int vigor;
};

std::string outOfcombatStaminaAob = "ba b4 00 00 00 48 8d 0d ?? ?? ?? ?? e8 ?? ?? ?? ?? 48 8b 0d ?? ?? ?? ?? e8 ?? ?? ?? ?? 84 c0 0f 94 c2 eb 02 32 d2 f6 c3 01 74 07 83 e3 fe";
int outOfcombatStaminaOffset = 31;

bool outOfCombatStamina = true;