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

float getCalcCorrectGraphHook(float value, int row){
    return getCalcCorrectGraph(value,row);
}


int getMaxHPHook(uintptr_t playerData){
    AttributeData attributeData = *(AttributeData*)(playerData+attributeDataOffset);
    for (auto [attribute,valuePtr] : iterOverAttributes(attributeData)){
        Log(attributeToString(attribute) + ": "+ std::to_string(*valuePtr));
    }
    float addHp = (getPlayerLevel(attributeData) - attributeData.vigor - 70) * (600.0 / (89.0*7.0));
    float finalHp =  getCalcCorrectGraph(attributeData.vigor,100);
	return finalHp + addHp;
}

int getMaxMPHook(uintptr_t playerData){
    AttributeData attributeData = *(AttributeData*)(playerData+attributeDataOffset);
	return getCalcCorrectGraph(attributeData.mind,101);
}

int getMaxSPHook(uintptr_t playerData){
    AttributeData attributeData = *(AttributeData*)(playerData+attributeDataOffset);
	return getCalcCorrectGraph(attributeData.endurance,104);
}

float getMaxEquipLoadHook(uintptr_t playerData){
    AttributeData attributeData = *(AttributeData*)(playerData+attributeDataOffset);
	return getCalcCorrectGraph(attributeData.endurance,220);
}