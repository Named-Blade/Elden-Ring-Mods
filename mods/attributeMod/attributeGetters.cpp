#include "attributeMod.hpp"


int getMaxHPByStatsHook(uintptr_t playerData){
    AttributeData attributeData = *(AttributeData*)(playerData+attributeDataOffset);
    Log(attributeData.vigor);
	return 1000;
}