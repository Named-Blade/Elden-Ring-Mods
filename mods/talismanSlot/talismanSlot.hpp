#pragma once
#include <iostream>
#include <Windows.h>
#include <algorithm>

#include <CallHook.h>
#include <PointerChain.h>

#include <ModUtils.hpp>
#include <config.hpp>

using namespace ModUtils;

std::string gameDataManAob = "48 8b 05 ?? ?? ?? ?? 48 85 c0 74 05 48 8b 40 58 c3 c3";
uintptr_t gameDataManPtr;
uintptr_t playerGameDataOffset = 0x8;
uintptr_t talismanSlotsOffset = 0xC6;
auto talismanSlots = PointerChain::make<int8_t, true>(gameDataManPtr,0x0,playerGameDataOffset,talismanSlotsOffset);

std::string enableSlotAob = "0f 8c ?? ?? ?? ?? c6 44 24 38 18 48 8d 4c 24 38 e8 ?? ?? ?? ?? 84 c0 74 11 48 8d 4d 00 e8 ?? ?? ?? ??";
int enableSlotOffset = 17;
int enableSlotOffset2 = 7;

std::string slotMaxAob = "0f b6 88 c6 00 00 00 41 0f b6 c0 03 c1 b9 04 00 00 00 3b c1 0f 47 c1 48 83 c4 20 5b";
int slotMaxOffset = 14;

std::string menuTypeAob = "c6 84 24 5a 01 00 00 01 48 8d 4c 24 30 48 89 4c 24 20 4c 8d 8c 24 58 01 00 00 4c 8b c0 48 8b d7 48 8b cb e8 ?? ?? ?? ?? 48 8b 9c 24 40 01 00 00 48 81 c4 30 01 00 00 5f c3";
int menuTypeOffset = 35;

std::string getMessageAob = "75 24 44 8b cb 33 d2 41 b8 9c 01 00 00 48 8b cf e8 ?? ?? ?? ?? 48 85 c0 48 8d 0d ?? ?? ?? ?? 48 0f 45 c8 48 8b c1 48 8b 5c 24 30";
int getMessageOffset = 17;
typedef const wchar_t * (*getMessageType) (uintptr_t,uint32_t,uint32_t,uint32_t);
getMessageType getMessageOriginal;

bool extraSlotOnMax = true;