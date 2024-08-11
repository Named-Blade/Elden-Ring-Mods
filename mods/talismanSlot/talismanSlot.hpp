#pragma once
#include <iostream>
#include <Windows.h>
#include <algorithm>

#include <dantelion2/system.hpp>

#include <ModUtils.hpp>
#include <config.hpp>

using namespace ModUtils;

std::string enableSlotAob = "0f 8c ?? ?? ?? ?? c6 44 24 38 18 48 8d 4c 24 38 e8 ?? ?? ?? ?? 84 c0 74 11 48 8d 4d 00 e8 ?? ?? ?? ??";
int enableSlotOffset = 17;
int enableSlotOffset2 = 7;

std::string slotMaxAob = "0f b6 88 c6 00 00 00 41 0f b6 c0 03 c1 b9 04 00 00 00 3b c1 0f 47 c1 48 83 c4 20 5b";
int slotMaxOffset = 14;

std::string getMessageAob = "75 24 44 8b cb 33 d2 41 b8 9c 01 00 00 48 8b cf e8 ?? ?? ?? ?? 48 85 c0 48 8d 0d ?? ?? ?? ?? 48 0f 45 c8 48 8b c1 48 8b 5c 24 30";
int getMessageOffset = 17;
typedef const wchar_t * (*getMessageType) (uintptr_t,uint32_t,uint32_t,uint32_t);
getMessageType getMessageOriginal;
