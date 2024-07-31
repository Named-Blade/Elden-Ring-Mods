#include "uncapMod.h"

// the return vavlue of this function is the current level cap
int getLevelCap(){
	return level_cap;
	
}
// the return value of this function is the current stat cap
//stat is equal to the current stat
int getStatCap(Stat stat){
	return stat_caps[0];
	
}
int levelCost(int level){
	int runes = levelCostOriginal(level);
	if (runes >= rune_cost_cap){
		return rune_cost_cap;
	}
	if (level >= levelMaxGrow){
		return levelCost(levelMaxGrow-1);
	}
	return runes;
}