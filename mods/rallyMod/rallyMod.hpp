#pragma once
#include <iostream>

#include <dantelion2/system.hpp>
#include <param/param.hpp>
#include <CallHook.h>

#include <ModUtils.hpp>
#include <config.hpp>

using namespace ModUtils;

std::string rallyTimeAob = "44 38 74 24 78 75 0d 0f 2f 8b 68 01 00 00 0f 82 a3 00 00 00 f3 0f 10 05 ?? ?? ?? ?? f3 0f 59 44 24 70 f3 0f 11 83 68 01 00 00 e9 88 00 00 00 8b 07 2b c1";
int rallyTimeOffset = 24;
int rallyTimeSize = 4;
float* rallyTime;

std::string rallyDecayAob = "f3 0f 10 93 64 01 00 00 0f 2f ca 76 4b 66 0f 6e 83 3c 01 00 00 0f 5b c0 f3 0f 59 05 ?? ?? ?? ?? f3 0f 59 c7 f3 0f 5c c8 f3 0f 5f ca 0f 2f f1 f3 0f 11 8b 60 01 00 00 76 05";
int rallyDecayOffset = 28;
int rallyDecaySize = 4;
float* rallyDecay;

std::string rallyHitAob = "e8 ?? ?? ?? ?? f3 0f 10 8b 60 01 00 00 0f 28 d0 f3 0f 5c ca 0f 2f f1 f3 0f 11 8b 60 01 00 00 76 0e f3 0f 5d b3 64 01 00 00";
int rallyHitOffset = 1;
int rallyHitSize = 4;

std::string rallyChangeAob = "e8 ?? ?? ?? ?? 84 c0 0f 84 ?? ?? ?? ?? 8b 8b 38 01 00 00 0f 57 c9 8b c1 2b c5 0f 2f f1 0f 86 ?? ?? ?? ?? 85 c0 0f 84";

float rallyConfigTime = 6.0;
float rallyConfigDecay = 60.0;
bool rallyResetOnHit = true;
bool rallyKeepOnHeal = true;
bool rallyResetOnHeal = true;

struct RallyData {
	float rallyPotential;
	float rallyCap;
	float rallyTimer;
};