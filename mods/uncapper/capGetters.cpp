#include "uncapMod.hpp"
#include "ds2Levels/ds2Levels.cpp"

// the return vavlue of this function is the current level cap
int getLevelCap(){
	return level_cap;
	
}
// the return value of this function is the current stat cap
//stat is equal to the current stat
int getStatCap(Stat stat){
	if (stat < 8){
		return stat_caps[stat];
	}
	return 0;
	
}

float getCalcCorrectGraph(float value, int rowId){
	auto [row, exists] = from::param::CalcCorrectGraph[rowId];
	if (exists && value > row.stageMaxVal4){
		float overGrow = (row.stageMaxGrowVal4 - row.stageMaxGrowVal3) / (row.stageMaxVal4 - row.stageMaxVal3);
		float growAmount = overGrow * (value - row.stageMaxVal4);
		return row.stageMaxVal4 + growAmount;
	}
    return getCalcCorrectGraphOriginal(value,rowId);
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