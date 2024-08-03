#pragma once
#include <iostream>

#include <dantelion2/system.hpp>
#include <param/param.hpp>
#include <CallHook.h>

#include <ModUtils.hpp>
#include <config.hpp>

using namespace ModUtils;

std::string outOfcombatStaminaAob = "ba b4 00 00 00 48 8d 0d ?? ?? ?? ?? e8 ?? ?? ?? ?? 48 8b 0d ?? ?? ?? ?? e8 ?? ?? ?? ?? 84 c0 0f 94 c2 eb 02 32 d2 f6 c3 01 74 07 83 e3 fe";
int outOfcombatStaminaOffset = 31;

bool outOfCombatStamina = true;