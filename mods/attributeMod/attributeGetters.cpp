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
	return getCalcCorrectGraphActual(attribute.vigor,100) + addHp;
}

int getMaxMP(PlayerParam &playerParam){
    AttributeData attribute = playerParam.attributeData;
	return getCalcCorrectGraphActual(attribute.mind,101);
}

int getMaxSP(PlayerParam &playerParam){
    AttributeData attribute = playerParam.attributeData;
	return getCalcCorrectGraphActual(attribute.endurance,104);
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
    defense.physical = (getCalcCorrectGraphActual(level,102) + getCalcCorrectGraphActual(attribute.strength,130)) * mult;
    defense.magic = (getCalcCorrectGraphActual(level,102) + getCalcCorrectGraphActual(attribute.intelligence,132)) * mult;
    defense.fire = (getCalcCorrectGraphActual(level,102) + getCalcCorrectGraphActual(attribute.vigor,133)) * mult;
    defense.lightning = (getCalcCorrectGraphActual(level,102) + ( (enduranceLightningDefense) ? getCalcCorrectGraphActual(attribute.endurance,134) : 0)) * mult;
    defense.holy = (getCalcCorrectGraphActual(level,102) + getCalcCorrectGraphActual(attribute.faith,135)) * mult;
}

void calcResist(ResistanceData &resistance, PlayerParam &playerParam){
    AttributeData attribute = playerParam.attributeData;
    
    int level = getPlayerLevel(attribute);
    resistance.poison = getCalcCorrectGraphActual(level,110) + getCalcCorrectGraphActual(attribute.vigor,120);
    resistance.scarletRot = getCalcCorrectGraphActual(level,111) + getCalcCorrectGraphActual(attribute.vigor,121);

    resistance.hemorrhage = getCalcCorrectGraphActual(level,112) + getCalcCorrectGraphActual(attribute.endurance,122);
    resistance.frostbite = getCalcCorrectGraphActual(level,113) + getCalcCorrectGraphActual(attribute.endurance,123);

    resistance.sleep = getCalcCorrectGraphActual(level,114) + getCalcCorrectGraphActual(attribute.mind,124);
    resistance.madness = getCalcCorrectGraphActual(level,115) + getCalcCorrectGraphActual(attribute.mind,125);

    resistance.death = getCalcCorrectGraphActual(level,116) + getCalcCorrectGraphActual(attribute.arcane,126);
}

float getMaxEquipLoad(PlayerParam &playerParam){
    AttributeData attribute = playerParam.attributeData;
	return getCalcCorrectGraphActual(attribute.endurance,220);
}

int getDiscovery(PlayerParam &playerParam){
    AttributeData attribute = playerParam.attributeData;
	return getCalcCorrectGraphActual(attribute.arcane,140)*100;
}

void calcStaminaRegen(HookContext* context, void* calcStaminaRegenFunc){
    calcStaminaRegenType calcStaminaRegenOrigin = (calcStaminaRegenType)calcStaminaRegenFunc;

    uintptr_t chrIns = context->rbx;
    auto spEffects = context->rcx;

    float& staminaRegen = context->imm0[0];
    staminaRegen = 0.0;

    staminaRegen += calcStaminaRegenOrigin(spEffects);
    staminaRegen += 45.0;
}