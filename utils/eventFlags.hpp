#pragma once

#include <cmath>
#include <ModUtils.hpp>

using namespace ModUtils;

struct EventFlagMan{
	void* ptr;
};

std::string getEventFlagValueAob = "44 8b 41 1c 44 8b da 33 d2 41 8b c3 41 f7 f0 4c 8b d1 45 33 c9 44 0f af c0 45 2b d8 4c 8b 41 38 49 8b d0 49 8b 48 08 44 38 49 19";
using eventFlagGetterType = bool(*)(EventFlagMan, unsigned int);
eventFlagGetterType eventFlagGetter;

std::string setEventFlagValueAob = "48 89 5c 24 08 44 8b 49 1c 44 8b d2 33 d2 41 8b c2 41 f7 f1 41 8b d8 4c 8b d9 4c 8b 41 38 44 0f af c8 49 8b c8 49 8b 50 08";
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
				eventFlagGetter = reinterpret_cast<eventFlagGetterType>(funcAddress);
			}
		}
		{
			uintptr_t funcAddress = AobScan(setEventFlagValueAob);
			if (funcAddress != 0)
			{
				eventFlagSetter = reinterpret_cast<eventFlagSetterType>(funcAddress);
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
		setEventFlagState(flagStart+i,(bool)(num & 1<<i));
	}
	Log(getEventFlagRange(flagStart,size));
}