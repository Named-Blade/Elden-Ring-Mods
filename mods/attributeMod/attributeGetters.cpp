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
std::vector<std::tuple<Attribute,int*>> iterOverAttributes(AttributeData &attribute){
    return std::vector<std::tuple<Attribute,int*>>{
        std::tuple<Attribute,int*>{Attribute::vigor,&attribute.vigor},
        std::tuple<Attribute,int*>{Attribute::mind,&attribute.mind},
        std::tuple<Attribute,int*>{Attribute::endurance,&attribute.endurance},
        std::tuple<Attribute,int*>{Attribute::strength,&attribute.strength},
        std::tuple<Attribute,int*>{Attribute::dexterity,&attribute.dexterity},
        std::tuple<Attribute,int*>{Attribute::intelligence,&attribute.intelligence},
        std::tuple<Attribute,int*>{Attribute::faith,&attribute.faith},
        std::tuple<Attribute,int*>{Attribute::arcane,&attribute.arcane}
    };
}

int getPlayerLevel(AttributeData attribute){
    return attribute.vigor +
        attribute.mind +
        attribute.endurance +
        attribute.strength +
        attribute.dexterity +
        attribute.intelligence +
        attribute.faith +
        attribute.arcane;
}

float getCalcCorrectGraph(float value, int row){
    return getCalcCorrectGraphOriginal(value,row);
}


int getMaxHP(PlayerParam &playerParam){
    AttributeData attribute = playerParam.attributeData;
    int level = getPlayerLevel(attribute);
    float addHp = hpBonusOnLevel ? (level - attribute.vigor - 70) * hpBonusPerLevel: 0;
	return getCalcCorrectGraph(attribute.vigor,100) + addHp;
}

int getMaxMP(PlayerParam &playerParam){
    AttributeData attribute = playerParam.attributeData;
	return getCalcCorrectGraph(attribute.mind,101);
}

int getMaxSP(PlayerParam &playerParam){
    AttributeData attribute = playerParam.attributeData;
    Log(attribute.endurance,": ",getCalcCorrectGraph(attribute.endurance,104));
	return getCalcCorrectGraph(attribute.endurance,104);
}

float calcDamageScale(PlayerParam &playerParam, getWeaponResult &weapon, uint64_t unk, DamageType damageType){
    float mult =  staminaDamageScaling ? minStaminaDamage + ((float)playerParam.stamina/(float)playerParam.maxStamina)*(maxStaminaDamage - minStaminaDamage) : 1;
    return calcDamageScaleOriginal(playerParam,weapon,unk,damageType) * mult;
}

void calcSpellScale(uintptr_t chrIns, spellScale &scaling, uint32_t weaponId){
    PlayerParam playerParam = getPlayerParam(chrIns);
    AttributeData attribute = playerParam.attributeData;
    calcSpellScaleOriginal(chrIns,scaling,weaponId);
}

void calcDefense(DefenseData &defense, PlayerParam &playerParam){
    AttributeData attribute = playerParam.attributeData;

    float mult =  staminaDefenseScaling ? minStaminaDefense + ((float)playerParam.stamina/(float)playerParam.maxStamina)*(maxStaminaDefense - minStaminaDefense) : 1;
    
    int level = getPlayerLevel(attribute);
    defense.physical = (getCalcCorrectGraph(level,102) + getCalcCorrectGraph(attribute.strength,130)) * mult;
    defense.magic = (getCalcCorrectGraph(level,102) + getCalcCorrectGraph(attribute.intelligence,132)) * mult;
    defense.fire = (getCalcCorrectGraph(level,102) + getCalcCorrectGraph(attribute.vigor,133)) * mult;
    defense.lightning = (getCalcCorrectGraph(level,102) + ( (enduranceLightningDefense) ? getCalcCorrectGraph(attribute.endurance,134) : 0)) * mult;
    defense.holy = (getCalcCorrectGraph(level,102) + getCalcCorrectGraph(attribute.faith,135)) * mult;
}

void calcResist(ResistanceData &resistance, PlayerParam &playerParam){
    AttributeData attribute = playerParam.attributeData;
    
    int level = getPlayerLevel(attribute);
    resistance.poison = getCalcCorrectGraph(level,110) + getCalcCorrectGraph(attribute.vigor,120);
    resistance.scarletRot = getCalcCorrectGraph(level,111) + getCalcCorrectGraph(attribute.vigor,121);

    resistance.hemorrhage = getCalcCorrectGraph(level,112) + getCalcCorrectGraph(attribute.endurance,122);
    resistance.frostbite = getCalcCorrectGraph(level,113) + getCalcCorrectGraph(attribute.endurance,123);

    resistance.sleep = getCalcCorrectGraph(level,114) + getCalcCorrectGraph(attribute.mind,124);
    resistance.madness = getCalcCorrectGraph(level,115) + getCalcCorrectGraph(attribute.mind,125);

    resistance.death = getCalcCorrectGraph(level,116) + getCalcCorrectGraph(attribute.arcane,126);
}

float getMaxEquipLoad(PlayerParam &playerParam){
    AttributeData attribute = playerParam.attributeData;
	return getCalcCorrectGraph(attribute.endurance,220);
}

int getDiscovery(PlayerParam &playerParam){
    AttributeData attribute = playerParam.attributeData;
	return getCalcCorrectGraph(attribute.arcane,140)*100;
}