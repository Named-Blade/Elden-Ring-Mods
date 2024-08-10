#pragma once
#include <iostream>

#include <param/param.hpp>
#include <dantelion2/system.hpp>

#include <ModUtils.hpp>
#include <config.hpp>
#include <eventFlags.hpp>

using namespace ModUtils;

enum Stat {vigor,mind,endurance,strength,dexterity,intelligence,faith,arcane};

std::vector<int> stat_caps(8,INT_MAX);
int level_cap = INT_MAX;

int levelMaxGrow = 0;
bool levelMaxFound = false;

std::string getCalcCorrectGraphAob = "66 0f 6e 83 88 02 00 00 0f 5b c0 ba 64 00 00 00 e8 ?? ?? ?? ?? f3 0f 2c c0 48 83 c4 50 5b c3";
int getCalcCorrectGraphOffset = 17;
int getCalcCorrectGraphSize = 4;

typedef float (*getCalcCorrectGraphType)(float, int);
float getCalcCorrectGraphDummy(float v, int r) {return 0;}
getCalcCorrectGraphType getCalcCorrectGraphOriginal = &getCalcCorrectGraphDummy;

extern "C"
{
	uintptr_t return_addresses [16];
	uintptr_t jump_address;
	uintptr_t jump_address_uncapped_level;
	
	void get_next_level();
	void get_current_stat();
	
	void compare_levels();
	uintptr_t self_locator();
	
	int levelCostOriginal(int level);
	
	int getLevelCap();
	int getStatCap(Stat);
}

int rune_cost_cap = 500'000;
bool adjustGraph = true;
bool ds2LevelCosts = true;

bool useEventFlags = false;
uint32_t levelCapRange = 0;
uint32_t vigorCapRange = 0;
uint32_t mindCapRange = 0;
uint32_t enduranceCapRange = 0;
uint32_t strengthCapRange = 0;
uint32_t dexterityCapRange = 0;
uint32_t intelligenceCapRange = 0;
uint32_t faithCapRange = 0;
uint32_t arcaneCapRange = 0;