#include "attributeMod.hpp"


enum Attribute {vigor, mind, endurance, strength, dexterity, intelligence, faith, arcane};
std::string attributeToString(Attribute attribute){
    static const std::map<Attribute, std::string> attributeMap = {
        {Attribute::vigor, "Vigor"},
        {Attribute::mind, "Mind"},
        {Attribute::endurance, "Endurance"},
        {Attribute::strength, "Strength"},
        {Attribute::dexterity, "Dexterity"},
        {Attribute::intelligence, "Intelligence"},
        {Attribute::faith, "Faith"},
        {Attribute::arcane, "Arcane"}
    };
    return attributeMap.at(attribute);
}
std::vector<std::tuple<Attribute,int*>> iterOverAttributes(AttributeData &attributeData){
    return std::vector<std::tuple<Attribute,int*>>{
        std::tuple<Attribute,int*>{Attribute::vigor,&attributeData.vigor},
        std::tuple<Attribute,int*>{Attribute::mind,&attributeData.mind},
        std::tuple<Attribute,int*>{Attribute::endurance,&attributeData.endurance},
        std::tuple<Attribute,int*>{Attribute::strength,&attributeData.strength},
        std::tuple<Attribute,int*>{Attribute::dexterity,&attributeData.dexterity},
        std::tuple<Attribute,int*>{Attribute::intelligence,&attributeData.intelligence},
        std::tuple<Attribute,int*>{Attribute::faith,&attributeData.faith},
        std::tuple<Attribute,int*>{Attribute::arcane,&attributeData.arcane}
    };
}

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

float getCalcCorrectGraph(float value, int row){
    return getCalcCorrectGraphOriginal(value,row);
}


int getMaxHP(uintptr_t playerData){
    AttributeData attributeData = *(AttributeData*)(playerData+attributeDataOffset);
	return getCalcCorrectGraph(attributeData.vigor,100);
}

int getMaxMP(uintptr_t playerData){
    AttributeData attributeData = *(AttributeData*)(playerData+attributeDataOffset);
	return getCalcCorrectGraph(attributeData.mind,101);
}

int getMaxSP(uintptr_t playerData){
    AttributeData attributeData = *(AttributeData*)(playerData+attributeDataOffset);
	return getCalcCorrectGraph(attributeData.endurance,104);
}

void calcDefense(DefenseData &defenseData, uintptr_t playerData){
    AttributeData attributeData = *(AttributeData*)(playerData+attributeDataOffset);
    for (auto [attribute,valuePtr] : iterOverAttributes(attributeData)){
        Log(attributeToString(attribute) + ": "+ std::to_string(*valuePtr));
    }
    int level = getPlayerLevel(attributeData);

    defenseData.physical = getCalcCorrectGraph(level,102) + getCalcCorrectGraph(attributeData.strength,130);
    defenseData.magic = getCalcCorrectGraph(level,102) + getCalcCorrectGraph(attributeData.intelligence,132);
    defenseData.fire = getCalcCorrectGraph(level,102) + getCalcCorrectGraph(attributeData.vigor,133);
    defenseData.lightning = getCalcCorrectGraph(level,102) + ( (enduranceLightningDefense) ? getCalcCorrectGraph(attributeData.endurance,134) : 0);
    defenseData.holy = getCalcCorrectGraph(level,102) + getCalcCorrectGraph(attributeData.faith,135);
}

float getMaxEquipLoad(uintptr_t playerData){
    AttributeData attributeData = *(AttributeData*)(playerData+attributeDataOffset);
	return getCalcCorrectGraph(attributeData.endurance,220);
}

int getDiscovery(uintptr_t playerData){
    AttributeData attributeData = *(AttributeData*)(playerData+attributeDataOffset);
	return getCalcCorrectGraph(attributeData.arcane,140)*100;
}