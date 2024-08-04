#include "attributeMod.hpp"

int getPlayerLevel(AttributeData attributeData){
    return attributeData.vigor +
        attributeData.mind +
        attributeData.endurance +
        attributeData.strength +
        attributeData.dexterity +
        attributeData.intelligence +
        attributeData.faith +
        attributeData.arcane;
}

float getCalcCorrectGraphHook(float value, int row){
    return getCalcCorrectGraph(value,row);
}


int getMaxHPByStatsHook(uintptr_t playerData){
    AttributeData attributeData = *(AttributeData*)(playerData+attributeDataOffset);
	return getCalcCorrectGraph(attributeData.vigor + ((getPlayerLevel(attributeData) - attributeData.vigor)/7)*0.35,100);
}