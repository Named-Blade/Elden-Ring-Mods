#include "uncapMod.hpp"
#include "ds2Levels/ds2Levels.cpp"

// the return vavlue of this function is the current level cap
int getLevelCap(){
	return level_cap;
	
}
// the return value of this function is the current stat cap
//stat is equal to the current stat
int getStatCap(Stat stat){
	return stat_caps[0];
	
}

void initLevels(){
	if (ds2LevelCosts){
		while (ds2LevelCost(levelMaxGrow) <= 999999999 && levelMaxGrow <= 99999){
			levelMaxGrow++;
		}
	} else {
		while (levelCostOriginal(levelMaxGrow) <= 999999999 && levelMaxGrow <= 99999){
			levelMaxGrow++;
		}
	}
	levelMaxFound = true;
}

int levelCost(int level){
	if (!levelMaxFound){
		initLevels();
	}
	int runes;
	if (ds2LevelCosts){
		runes = ds2LevelCost(level);
		if (level >= levelMaxGrow){
			runes = 999999999;
		}
		if (runes >= rune_cost_cap){
			return rune_cost_cap;
		}
	} else {
		runes = levelCostOriginal(level);
		if (level >= levelMaxGrow){
			runes = 999999999;
		}
		if (runes >= rune_cost_cap){
			return rune_cost_cap;
		}
	}
	return runes;
}