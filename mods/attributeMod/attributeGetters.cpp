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


int getMaxHP(PlayerData* playerData){
    AttributeData attributeData = (*playerData).attributeData;
    int level = getPlayerLevel(attributeData);
    float addHp = hpBonusOnLevel ? (level - attributeData.vigor - 70) * (600.0/(89.0*7.0)): 0;
	return getCalcCorrectGraph(attributeData.vigor,100) + addHp;
}

int getMaxMP(PlayerData* playerData){
    AttributeData attributeData = (*playerData).attributeData;
	return getCalcCorrectGraph(attributeData.mind,101);
}

int getMaxSP(PlayerData* playerData){
    AttributeData attributeData = (*playerData).attributeData;
	return getCalcCorrectGraph(attributeData.endurance,104);
}

float calcDamageScale(getWeaponResult* weapon, PlayerData* playerData, uint64_t unk, DamageType damageType){
    return calcDamageScaleOriginal(weapon,playerData,unk,damageType);
}

void calcSpellScale(uintptr_t chrIns, spellScale &scaling, uint32_t weaponId){
    calcSpellScaleOriginal(chrIns,scaling,weaponId);
}

void calcDefense(DefenseData &defenseData, PlayerData* playerData){
    AttributeData attributeData = (*playerData).attributeData;
    
    int level = getPlayerLevel(attributeData);
    defenseData.physical = getCalcCorrectGraph(level,102) + getCalcCorrectGraph(attributeData.strength,130);
    defenseData.magic = getCalcCorrectGraph(level,102) + getCalcCorrectGraph(attributeData.intelligence,132);
    defenseData.fire = getCalcCorrectGraph(level,102) + getCalcCorrectGraph(attributeData.vigor,133);
    defenseData.lightning = getCalcCorrectGraph(level,102) + ( (enduranceLightningDefense) ? getCalcCorrectGraph(attributeData.endurance,134) : 0);
    defenseData.holy = getCalcCorrectGraph(level,102) + getCalcCorrectGraph(attributeData.faith,135);
}

void calcResist(ResistanceData &resistanceData, PlayerData* playerData){
    AttributeData attributeData = (*playerData).attributeData;
    
    int level = getPlayerLevel(attributeData);
    resistanceData.poison = getCalcCorrectGraph(level,110) + getCalcCorrectGraph(attributeData.vigor,120);
    resistanceData.scarletRot = getCalcCorrectGraph(level,111) + getCalcCorrectGraph(attributeData.vigor,121);

    resistanceData.hemorrhage = getCalcCorrectGraph(level,112) + getCalcCorrectGraph(attributeData.endurance,122);
    resistanceData.frostbite = getCalcCorrectGraph(level,113) + getCalcCorrectGraph(attributeData.endurance,123);

    resistanceData.sleep = getCalcCorrectGraph(level,114) + getCalcCorrectGraph(attributeData.mind,124);
    resistanceData.madness = getCalcCorrectGraph(level,115) + getCalcCorrectGraph(attributeData.mind,125);

    resistanceData.death = getCalcCorrectGraph(level,116) + getCalcCorrectGraph(attributeData.arcane,126);
}

float getMaxEquipLoad(PlayerData* playerData){
    AttributeData attributeData = (*playerData).attributeData;
	return getCalcCorrectGraph(attributeData.endurance,220);
}

int getDiscovery(PlayerData* playerData){
    AttributeData attributeData = (*playerData).attributeData;
	return getCalcCorrectGraph(attributeData.arcane,140)*100;
}