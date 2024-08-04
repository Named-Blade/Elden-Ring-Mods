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
    float addHp = (getPlayerLevel(attributeData) - attributeData.vigor - 70) * (600.0 / (89.0*7.0));
    float finalHp =  getCalcCorrectGraph(attributeData.vigor,100);
	return finalHp + addHp;
}