#include "uncapMod.hpp"
#include "ds2Levels/ds2Levels.cpp"


int getLevelCap(){
	if(useEventFlags && levelCapRange != 0){
		int levelCap = getEventFlagRange(levelCapRange,32);
		if (levelCap != 0){
			return levelCap;
		}
	}
	return level_cap;
}

int getStatCapFlag(Stat stat){
	switch(stat){
		case Stat::vigor:
			if (vigorCapRange != 0) return getEventFlagRange(vigorCapRange,32);
			break;
		case Stat::mind:
			if (mindCapRange != 0) return getEventFlagRange(mindCapRange,32);
			break;
		case Stat::endurance:
			if (enduranceCapRange != 0) return getEventFlagRange(enduranceCapRange,32);
			break;
		case Stat::strength:
			if (strengthCapRange != 0) return getEventFlagRange(strengthCapRange,32);
			break;
		case Stat::dexterity:
			if (dexterityCapRange != 0) return getEventFlagRange(dexterityCapRange,32);
			break;
		case Stat::intelligence:
			if (intelligenceCapRange != 0) return getEventFlagRange(intelligenceCapRange,32);
			break;
		case Stat::faith:
			if (faithCapRange != 0) return getEventFlagRange(faithCapRange,32);
			break;
		case Stat::arcane:
			if (arcaneCapRange != 0) return getEventFlagRange(arcaneCapRange,32);
			break;
	}
	return 0;
}

int getStatCap(Stat stat){
	if(useEventFlags){
		int statCap = getStatCapFlag(stat);
		if (statCap != 0){
			return statCap;
		}
	}
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