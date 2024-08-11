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

std::string getMessageAob = "75 24 44 8b cb 33 d2 41 b8 9c 01 00 00 48 8b cf e8 ?? ?? ?? ?? 48 85 c0 48 8d 0d ?? ?? ?? ?? 48 0f 45 c8 48 8b c1 48 8b 5c 24 30";
int getMessageOffset = 17;
typedef const wchar_t * (*getMessageType) (uintptr_t,uint32_t,uint32_t,uint32_t);
getMessageType getMessageOriginal;
