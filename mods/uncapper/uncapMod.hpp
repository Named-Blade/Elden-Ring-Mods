#pragma once
#include <iostream>

#include <param/param.hpp>
#include <dantelion2/system.hpp>

#include <ModUtils.hpp>
#include <config.hpp>

using namespace ModUtils;

enum Stat {vigor,mind,endurance,strength,dexterity,intelligence,faith,arcane};

std::vector<int> stat_caps(8,INT_MAX);
int level_cap = INT_MAX;

int levelMaxGrow = 0;

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

int rune_cost_cap = 100000;
bool adjustGraph = true;