#pragma once

#include <algorithm>
#include <Windows.h>

#include <dantelion2/reflection.hpp>
#include <dantelion2/system.hpp>
#include <detail/base_address.hpp>
#include <detail/symbols.hpp>
#include <coresystem/cs_param.hpp>
#include <coresystem/task.hpp>
#include <param/param.hpp>

#include <config.hpp>

uintptr_t worldCharManPtr = (uintptr_t)liber::symbol<"CS::WorldChrMan::instance">::get();
uintptr_t localPlayerOffset = 0x10EF8;
uintptr_t playerInstanceOffset = 0x0;
uintptr_t playerModulesOffset = 0x0190;
uintptr_t playerDataOffset = 0x0;
uintptr_t maxHealthOffset = +0x013C;
uintptr_t maxHealthDDOffset = +0x0140;
uintptr_t maxFocusOffset = +0x014C;
uintptr_t maxStaminaOffset = +0x0158;

int maxHealthParam = 0;
int maxStaminaParam = 0;
int maxFocusParam = 0;

struct statBar{
    bool isLinearFirst = false;
    float maxDisplaySize = 1;
    bool squishDisplay = false;
};

statBar healthConfig;
statBar focusConfig;
statBar staminaConfig;