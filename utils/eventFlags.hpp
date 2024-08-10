#pragma once

#include <cmath>
#include <ModUtils.hpp>

using namespace ModUtils;

struct EventFlagMan{
	void* ptr;
};

std::string getEventFlagValueAob = "e9 ?? ?? ?? ?? 48 83 ec 28 8b 12 85 d2 74 0f e8 ?? ?? ?? ?? 85 c0 0f 95 c0 48 83 c4 28 c3";
int getEventFlagValueOffset = 16;
using eventFlagGetterType = bool(*)(EventFlagMan, unsigned int);
eventFlagGetterType eventFlagGetter;

std::string setEventFlagValueAob = "e8 ?? ?? ?? ?? 8b 13 48 8b ce 44 0f b6 c7 e8 ?? ?? ?? ?? 80 7c 24 60 00 74 5c 8b 03 4c 8d 4c 24 48 48 8b 9e c8 00 00 00";
int setEventFlagValueOffset = 15;
using eventFlagSetterType = bool(*)(EventFlagMan, unsigned int, bool);
eventFlagSetterType eventFlagSetter;

std::string eventFlagManAob = "48 8d 0d 6d c9 44 02 e8 28 de 93 01 48 8b 1d 29 d3 7e 03 48 8b d6 48 8d 4c 24 20 e8 24 ce 05 00";
int eventFlagManAobOffset = 15;
int eventFlagManInstructionSize = 4;
EventFlagMan* eventFlagMan;

bool isEventFlagInit = false;

void initFlagHandlers(){
	
		{
			uintptr_t funcAddress = AobScan(getEventFlagValueAob);
			if (funcAddress != 0)
			{
				eventFlagGetter = reinterpret_cast<eventFlagGetterType>(getAddressFromMemory(funcAddress,getEventFlagValueOffset,4));
			}
		}
		{
			uintptr_t funcAddress = AobScan(setEventFlagValueAob);
			if (funcAddress != 0)
			{
				eventFlagSetter = reinterpret_cast<eventFlagSetterType>(getAddressFromMemory(funcAddress,setEventFlagValueOffset,4));
			}
		}
		{
			uintptr_t address = AobScan(eventFlagManAob);
			
			if (address != 0)
			{
				eventFlagMan = (EventFlagMan*)getAddressFromMemory(address,eventFlagManAobOffset,eventFlagManInstructionSize);
			}
		}
		
		isEventFlagInit = true;
}

bool getEventFlagState (unsigned int flag)
{
	if (!isEventFlagInit){
		initFlagHandlers();
	}
	
	if (eventFlagMan != nullptr && eventFlagGetter != nullptr){
		return eventFlagGetter(*eventFlagMan,flag);
	} else {
		return false;
	}
}

unsigned int getEventFlagRange(unsigned int flagStart,unsigned int size){
	unsigned int num = 0;
	for (int i = 0; i < size; i++){
		num += (unsigned int)getEventFlagState(flagStart+i) * pow(2,i);
	}
	return num;
}

void setEventFlagState (unsigned int flag, bool state){
	if (!isEventFlagInit){
		initFlagHandlers();
	}
	
	if (eventFlagMan != nullptr && eventFlagSetter != nullptr){
		eventFlagSetter(*eventFlagMan,flag,state);
	}
}

void setEventFlagRange(unsigned int flagStart,unsigned int size,unsigned int num){
	for (int i = 0; i < size; i++){
		if ((num & 1<<i) > 0){
			setEventFlagState(flagStart+i,true);
		} else {
			setEventFlagState(flagStart+i,false);
		}
	}
	Log(getEventFlagRange(flagStart,size));
}