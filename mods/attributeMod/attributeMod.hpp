#pragma once
#include <iostream>

#include <dantelion2/system.hpp>
#include <param/param.hpp>
#include <CallHook.h>

#include <ModUtils.hpp>
#include <config.hpp>
#pragma pack(push, 1)

using namespace ModUtils;

struct AttributeData{
    int32_t vigor;
    int32_t mind;
    int32_t endurance;
    int32_t forbiddenMysteryStat;
    int32_t strength;
    int32_t dexterity;
    int32_t intelligence;
    int32_t faith;
    int32_t arcane;
};
struct PlayerParam{
    char8_t _1[0x10];
    int32_t health;
    int32_t maxHealth;
    int32_t maxHealthBase;
    int32_t focus;
    int32_t maxFocus;
    int32_t maxFocusBase;
    char8_t _2[0x4];
    int32_t stamina;
    int32_t maxStamina;
    int32_t maxStaminaBase;
    char8_t _3[0x250];
    AttributeData attributeData;
};

typedef PlayerParam& (*getPlayerParamType)(uintptr_t);
PlayerParam& getPlayerParam(uintptr_t chrIns){
    getPlayerParamType getPlayerParamFunc = *(getPlayerParamType*)((*(uintptr_t*)chrIns) + 0x168);
    return getPlayerParamFunc(chrIns);
}


std::string getCalcCorrectGraphAob = "66 0f 6e 83 88 02 00 00 0f 5b c0 ba 64 00 00 00 e8 ?? ?? ?? ?? f3 0f 2c c0 48 83 c4 50 5b c3";
int getCalcCorrectGraphOffset = 17;
int getCalcCorrectGraphSize = 4;

typedef float (*getCalcCorrectGraphType)(float, int);
float getCalcCorrectGraphDummy(float v, int r) {return 0;}
getCalcCorrectGraphType getCalcCorrectGraphOriginal = &getCalcCorrectGraphDummy;
getCalcCorrectGraphType getCalcCorrectGraphActual = &getCalcCorrectGraphDummy;

std::string getMaxHPAob = "41 89 4e 08 8b 4e 1c 41 89 4e 10 48 8b ce e8 ?? ?? ?? ?? 8b c8 49 8b d4 e8 ?? ?? ?? ?? f3 0f 2c c8 41 89 4e 0c 48 8b ce";
int getMaxHPOffset = 15;
int getMaxHPSize = 4;

std::string getMaxMPAob = "e8 ?? ?? ?? ?? f3 0f 2c c8 41 89 4e 0c 48 8b ce e8 ?? ?? ?? ?? 8b c8 49 8b d4 e8 ?? ?? ?? ?? f3 0f 2c c0 48 8b ce";
int getMaxMPOffset = 17;
int getMaxMPSize = 4;

std::string getMaxSPAob = "e8 ?? ?? ?? ?? f3 0f 2c c0 48 8b ce 41 89 46 14 e8 ?? ?? ?? ?? 8b c8 49 8b d4 e8 ?? ?? ?? ?? f3 0f 2c c0 ba 73 00 00 00";
int getMaxSPOffset = 17;
int getMaxSPSize = 4;

std::string calcDamageScaleAob = "44 0f 28 de 45 33 c9 48 8d 54 24 78 4c 8b c7 49 8b cf e8 ?? ?? ?? ?? 0f b6 4f 44 44 0f 28 e8 44 0f 28 d6 8b d1 85 c9";
int calcDamageScaleOffset = 19;
int calcDamageScaleSize = 4;

enum DamageType {physical,magic,fire,lightning,holy};
struct getWeaponResult{
	uint32_t id;
	char8_t _1[0x4];
	from::paramdef::EQUIP_PARAM_WEAPON_ST *row;
	uint32_t baseId;
	char8_t _2[0x4];
	uint32_t reinforceId;
	char8_t _3[0x4];
	from::paramdef::REINFORCE_PARAM_WEAPON_ST *reinforceRow;
	char8_t _4[0x4];
};

typedef float (*calcDamageScaleType)(PlayerParam&, getWeaponResult&, uint64_t, DamageType);
float calcDamageScaleDummy(PlayerParam &_1, getWeaponResult&_2, uint64_t _4,DamageType _3) {return 0;}
calcDamageScaleType calcDamageScaleOriginal = &calcDamageScaleDummy;

std::string calcSpellScaleAob = "48 8b d9 41 8b f8 48 8b ca 48 8b f2 e8 ?? ?? ?? ?? 48 8b 03 48 8b cb ff 90 68 01 00 00 48 8b c8 8b d7 e8 ?? ?? ?? ?? f3 0f 11 06 48 8b cb 48 8b 03 ff 90 68 01 00 00";
int calcSpellScaleOffset = -15;

struct spellScale{
    float intelligence;
    float faith;
    float strength;
    float dexterity;
};

typedef void (*calcSpellScaleType)(uintptr_t,spellScale &,uint32_t);
void calcSpellScaleDummy(uintptr_t chrIns, spellScale &scaling, uint32_t weaponId) {}
calcSpellScaleType calcSpellScaleOriginal = &calcSpellScaleDummy;

std::string calcDefenseAob = "44 0f 29 50 88 4c 8b e2 0f 11 45 87 c7 45 97 00 00 00 00 0f 57 f6 e8 ?? ?? ?? ?? 48 8b 4d 77 0f 28 05 ?? ?? ?? ?? 0f 28 0d ?? ?? ?? ?? 48 c7 45 bf 00 00 00 00";
int calcDefenseOffset = 23;
int calcDefenseSize = 4;

struct DefenseData{
    float physical;
    float magic;
    float fire;
    float lightning;
    float holy;
};

std::string calcResistAob = "48 8d 48 98 0f 11 40 98 48 c7 40 a8 00 00 00 00 c7 40 b0 00 00 00 00 e8 ?? ?? ?? ?? 45 0f 28 d1 48 85 db 74 1a 44 0f b6 8c 24 b8 00 00 00 4c 8b c5";
int calcResistOffset = 24;
int calcResistSize = 4;

struct ResistanceData{
    float poison;
    float scarletRot;
    float hemorrhage;
    float death;
    float frostbite;
    float sleep;
    float madness;
};

std::string getMaxEquipAob = "f3 41 0f 11 46 1c e8 ?? ?? ?? ?? 48 8b ce 0f 28 f0 e8 ?? ?? ?? ?? f3 0f 59 f0 44 0f b6 cb 44 0f b6 c7 4c 89 7c 24 20 f3 41 0f 10 46 1c";
int getMaxEquipOffset = 18;
int getMaxEquipSize = 4;

std::string getDiscoveryAob = "48 8b 48 48 e8 ?? ?? ?? ?? f3 0f 11 87 6c 0a 00 00 48 8b cf 48 8b 96 78 01 00 00 e8 ?? ?? ?? ?? 48 8b 8f 30 05 00 00 4c 8b bc 24 f0 00 00 00";
int getDiscoveryOffset = 28;
int getDiscoverySize = 4;

std::string outOfcombatStaminaAob = "ba b4 00 00 00 48 8d 0d ?? ?? ?? ?? e8 ?? ?? ?? ?? 48 8b 0d ?? ?? ?? ?? e8 ?? ?? ?? ?? 84 c0 0f 94 c2 eb 02 32 d2 f6 c3 01 74 07 83 e3 fe";
int outOfcombatStaminaOffset = 31;

std::string staminaEquipAob = "41 83 ef 03 74 1a 41 83 ff 01 74 0a f3 0f 10 05 ?? ?? ?? ?? eb 12 f3 0f 10 05 ?? ?? ?? ?? eb 08 f3 0f 10 05 ?? ?? ?? ?? 0f b6 05 ?? ?? ?? ?? 84 c0 74 04 44 0f b6 f0 48 8b 46 58";
int staminaEquipOffset = 4;

std::string calcStaminaRegenAob = "40 53 48 83 ec 20 48 8b d9 48 8b 89 78 01 00 00 e8 ?? ?? ?? ?? 48 8b 83 90 01 00 00 f3 0f 10 15 ?? ?? ?? ?? f3 0f 58 d0 48 8b 08 8b 91 58 01 00 00 8b 89 54 01 00 00 48 83 c4 20";
int calcStaminaRegenOffset = 17;
int calcStaminaRegenDefaultOffset = 32;
typedef float (*calcStaminaRegenType)(uintptr_t);


bool outOfCombatStamina = true;
bool enduranceLightningDefense = true;
bool hpBonusOnLevel = true;
float hpBonusPerLevel = 900.0/(89.0*7.0);
bool staminaDamageScaling = true;
float minStaminaDamage = 0.9;
float maxStaminaDamage = 1.1;
bool staminaDefenseScaling = true;
float minStaminaDefense = 0.9;
float maxStaminaDefense = 1.1;

#pragma pack(pop)