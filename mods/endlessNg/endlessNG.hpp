#pragma once

#include <algorithm>
#include <Windows.h>
#include <utility>

#include <dantelion2/reflection.hpp>
#include <dantelion2/system.hpp>
#include <detail/base_address.hpp>
#include <detail/symbols.hpp>
#include <coresystem/cs_param.hpp>
#include <coresystem/task.hpp>
#include <param/param.hpp>

#include <ModUtils.hpp>
#include <config.hpp>
#include <eventFlags.hpp>

bool fixStandardDamage = true;

//event flag handling
unsigned int flagCheckNewCycle = 1051439330;
unsigned int flagSignNewCycle = 1051439331;
unsigned int flagStartNewCycle = 1051439332;
//22021100,1,2,3 <-- talk id
//67350 <-- item id

//aob sourced from erdtools by nordgaren
std::string gameDataManAob = "48 8b 05 ?? ?? ?? ?? 48 85 c0 74 05 48 8b 40 58 c3 c3";
uintptr_t gameDataManPtr;
uintptr_t ngCycleOffset = 0x120;

std::string healthCapAob = "eb 14 81 fa ff ff 07 00 48 8d 44 24 18 4c 8d 44 24 10 49 0f 4e c0 8b 10 89 91 3c 01 00 00";
int healthCapOffset = 18;
std::string ngLoadCapAob = "81 f9 0f 27 00 00 48 8d 45 f0 48 8d 55 f8 48 0f 4e c2 8b 08 48 8b 05 7b 71 b0 03 89 88 20 01 00 00 84 db";
int ngLoadCapOffset = 14;
std::string ngNewCapAob = "81 f9 0f 27 00 00 48 8d 44 24 38 4c 8d 44 24 40 49 0f 4e c0 8b 00 48 8d 4c 24 30 89 82 20 01 00 00";
int ngNewCapOffset = 16;
std::string loopCountAob = "c7 45 77 0f 27 00 00 c7 45 6f 00 00 00 00 79 06 48 8d 45 6f eb 12 81 f9 0f 27 00 00 48 8d 45 77 48 8d 55 e7 48 0f 4e c2";
int loopCountOffset = 36;
std::string nextLoopCountAob1 = "3d 0f 27 00 00 48 8d 4c 24 38 48 8d 54 24 40 48 0f 4e ca 8b 09 83 c1 01 c7 44 24 38 0f 27 00 00";
int nextLoopCountOffset1 = 15;
std::string nextLoopCountAob2 = "81 f9 0f 27 00 00 48 8d 44 24 38 48 8d 54 24 40 48 0f 4e c2 8b 00 48 83 c4 28 c3 90";
int nextLoopCountOffset2 = 16;

from::paramdef::CLEAR_COUNT_CORRECT_PARAM_ST cycleIncrease{};
from::paramdef::CLEAR_COUNT_CORRECT_PARAM_ST originalMax{};

std::string getGoodsAob = "8b 12 48 8d 4d c0 83 cf ff 89 75 c4 89 7d c0 48 89 75 c8 e8 ?? ?? ?? ?? 48 8b 45 c8 48 85 c0 74 03 8b 78 74 8b c7 eb 32";
int getGoodsOffset = 20;

#pragma pack(push, 1)
struct GetGoodsResult{
    uint32_t id;
    char8_t _1[0x4];
    from::paramdef::EQUIP_PARAM_GOODS_ST *row;
};
#pragma pack(pop)

typedef void (*getGoodsType) (GetGoodsResult&,uint32_t);
getGoodsType getGoodsOriginal;

//not modifying poise(SuperArmorDamageRate) as that would lead to infinite poise, which isn't very fun
#define CLEAR_COUNT_CORRECT_PARAM_FIELDS \
    X(MaxHpRate) \
    X(MaxMpRate) \
    X(MaxStaminaRate) \
    X(PhysicsAttackRate) \
    X(SlashAttackRate) \
    X(BlowAttackRate) \
    X(ThrustAttackRate) \
    X(NeturalAttackRate) \
    X(MagicAttackRate) \
    X(FireAttackRate) \
    X(ThunderAttackRate) \
    X(DarkAttackRate) \
    X(PhysicsDefenseRate) \
    X(MagicDefenseRate) \
    X(FireDefenseRate) \
    X(ThunderDefenseRate) \
    X(DarkDefenseRate) \
    X(StaminaAttackRate) \
    X(SoulRate) \
    X(PoisionResistRate) \
    X(DiseaseResistRate) \
    X(BloodResistRate) \
    X(CurseResistRate) \
    X(FreezeResistRate) \
    X(BloodDamageRate) \
    X(FreezeDamageRate) \
    X(SleepResistRate) \
    X(MadnessResistRate) \
    X(SleepDamageRate) \
    X(MadnessDamageRate)