#pragma once
#include <iostream>

#include <dantelion2/system.hpp>
#include <param/param.hpp>

#include <ModUtils.hpp>
#include <config.hpp>

using namespace ModUtils;

std::string getCalcCorrectGraphAob = "66 0f 6e 83 88 02 00 00 0f 5b c0 ba 64 00 00 00 e8 ?? ?? ?? ?? f3 0f 2c c0 48 83 c4 50 5b c3";
int getCalcCorrectGraphOffset = 17;
int getCalcCorrectGraphSize = 4;

typedef float (*getCalcCorrectGraphType)(float, int);
float getCalcCorrectGraphDummy(float v, int r) {return 0;}
getCalcCorrectGraphType getCalcCorrectGraph = &getCalcCorrectGraphDummy;

std::string getMaxHPAob = "41 89 4e 08 8b 4e 1c 41 89 4e 10 48 8b ce e8 ?? ?? ?? ?? 8b c8 49 8b d4 e8 ?? ?? ?? ?? f3 0f 2c c8 41 89 4e 0c 48 8b ce";
int getMaxHPOffset = 15;
int getMaxHPSize = 4;

std::string getMaxMPAob = "e8 ?? ?? ?? ?? f3 0f 2c c8 41 89 4e 0c 48 8b ce e8 ?? ?? ?? ?? 8b c8 49 8b d4 e8 ?? ?? ?? ?? f3 0f 2c c0 48 8b ce";
int getMaxMPOffset = 17;
int getMaxMPSize = 4;

std::string getMaxSPAob = "e8 ?? ?? ?? ?? f3 0f 2c c0 48 8b ce 41 89 46 14 e8 ?? ?? ?? ?? 8b c8 49 8b d4 e8 ?? ?? ?? ?? f3 0f 2c c0 ba 73 00 00 00";
int getMaxSPOffset = 17;
int getMaxSPSize = 4;

std::string getMaxEquipAob = "f3 41 0f 11 46 1c e8 ?? ?? ?? ?? 48 8b ce 0f 28 f0 e8 ?? ?? ?? ?? f3 0f 59 f0 44 0f b6 cb 44 0f b6 c7 4c 89 7c 24 20 f3 41 0f 10 46 1c";
int getMaxEquipOffset = 18;
int getMaxEquipSize = 4;

std::string getDiscoveryAob = "48 8b 48 48 e8 ?? ?? ?? ?? f3 0f 11 87 6c 0a 00 00 48 8b cf 48 8b 96 78 01 00 00 e8 ?? ?? ?? ?? 48 8b 8f 30 05 00 00 4c 8b bc 24 f0 00 00 00";
int getDiscoveryOffset = 28;
int getDiscoverySize = 4;

int attributeDataOffset = 0x288;
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

std::string outOfcombatStaminaAob = "ba b4 00 00 00 48 8d 0d ?? ?? ?? ?? e8 ?? ?? ?? ?? 48 8b 0d ?? ?? ?? ?? e8 ?? ?? ?? ?? 84 c0 0f 94 c2 eb 02 32 d2 f6 c3 01 74 07 83 e3 fe";
int outOfcombatStaminaOffset = 31;

bool outOfCombatStamina = true;