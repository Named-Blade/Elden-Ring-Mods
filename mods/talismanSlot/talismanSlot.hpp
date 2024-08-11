#pragma once
#include <iostream>
#include <Windows.h>
#include <algorithm>

#include <dantelion2/system.hpp>

#include <ModUtils.hpp>
#include <config.hpp>

using namespace ModUtils;

std::string enableSlotAob = "0f 8c ?? ?? ?? ?? c6 44 24 38 18 48 8d 4c 24 38 e8 ?? ?? ?? ?? 84 c0 74 11 48 8d 4d 00 e8 ?? ?? ?? ??";
int enableSlotOffset = 23;
