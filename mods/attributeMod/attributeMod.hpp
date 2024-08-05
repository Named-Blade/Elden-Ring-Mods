#pragma once
#include <iostream>

#include <dantelion2/system.hpp>
#include <param/param.hpp>

#include <ModUtils.hpp>
#include <config.hpp>

using namespace ModUtils;

const int attributeDataOffset = 0x288;
struct AttributeData{
    int vigor;
    int mind;
    int endurance;
    int forbiddenMysteryStat;
    int strength;
    int dexterity;
    int intelligence;
    int faith;
    int arcane;
};
struct PlayerData{
    char8_t _1[attributeDataOffset];
    AttributeData attributeData;
};


std::string getCalcCorrectGraphAob = "66 0f 6e 83 88 02 00 00 0f 5b c0 ba 64 00 00 00 e8 ?? ?? ?? ?? f3 0f 2c c0 48 83 c4 50 5b c3";
int getCalcCorrectGraphOffset = 17;
int getCalcCorrectGraphSize = 4;

typedef float (*getCalcCorrectGraphType)(float, int);
float getCalcCorrectGraphDummy(float v, int r) {return 0;}
getCalcCorrectGraphType getCalcCorrectGraphOriginal = &getCalcCorrectGraphDummy;

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
	unsigned int id;
	int _1;
	from::paramdef::EQUIP_PARAM_WEAPON_ST *row;
	unsigned int baseId;
	int _2;
	unsigned int reinforceId;
	int _3;
	from::paramdef::REINFORCE_PARAM_WEAPON_ST *reinforceRow;
	int _4;
};

typedef float (*calcDamageScaleType)(getWeaponResult*, PlayerData*, uint64_t, DamageType);
float calcDamageScaleDummy(getWeaponResult* _1, PlayerData* _2, uint64_t _4,DamageType _3) {return 0;}
calcDamageScaleType calcDamageScaleOriginal = &calcDamageScaleDummy;

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

bool outOfCombatStamina = true;
bool enduranceLightningDefense = true;
bool hpBonusOnLevel = true;