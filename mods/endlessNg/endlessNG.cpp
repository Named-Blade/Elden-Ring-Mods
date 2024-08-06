#include "endlessNg.hpp"
#include "DLLMain.hpp"

using namespace ModUtils;

template <typename T>
T ptrChain (std::vector<uintptr_t> ptrs, T defaultArg){
	if (ptrs.empty()) {
		return defaultArg;
	}
	uintptr_t current = ptrs[0];
	for (size_t i = 1; i < ptrs.size()-1; ++i){
		if (current == 0){
			return defaultArg;
		}
		current = *reinterpret_cast<uintptr_t*>(current + ptrs[i]);
	}
	if (current == 0){
		return defaultArg;
	}
	T result = *reinterpret_cast<T*>(current + ptrs.back());
	return result; 
}

unsigned int getCurrentCycle(){
	std::vector<uintptr_t> ptrs = {gameDataManPtr,0x0,ngCycleOffset};
	return ptrChain<unsigned int>(ptrs,0);
}
void setCurrentCycle(int cycleNum){
	std::vector<uintptr_t> ptrs = {gameDataManPtr,0x0};
	unsigned int* cycle = (unsigned int*)(ptrChain<uintptr_t>(ptrs,0)+ngCycleOffset);
	*cycle = cycleNum;
}

from::paramdef::EQUIP_PARAM_GOODS_ST displayGoods;
bool initDisplay = false;

void getGoods(GetGoodsResult &result, uint32_t id){
	if (id == 67350){
		getGoodsOriginal(result,2912);
		result.id = 67350;
		if (!initDisplay) {
			displayGoods = *(result.row);
			displayGoods.maxNum = 9999;
			initDisplay = true;
		}
		result.row = &displayGoods;
	} else {
		getGoodsOriginal(result,id);
	}
}

const wchar_t * getMessage(uintptr_t messageRepository, uint32_t _1,uint32_t msgBnd, uint32_t msgId){
	if (msgId == 67350 && msgBnd == 10){
		return L"Modify Intensity By:";
	}
	if (msgBnd == 33){
		if (msgId == 22021100){
			return L"Increase Intensity (Current: <?loopCount?>)";
		}
		if (msgId == 22021101){
			return L"Decrease Intensity (Current: <?loopCount?>)";
		}
		if (msgId == 22021102){
			return L"Current Intensity: <?loopCount?>";
		}
		if (msgId == 22021103){
			return L"Intensity Updated";
		}
	}
	return getMessageOriginal(messageRepository,_1,msgBnd,msgId);
}

struct set_ng_cycle_task: public from::CS::CSEzTask {
	void eztask_execute(from::FD4::FD4TaskData* data) override {
		
		if (getEventFlagState(flagCheckNewCycle)){
			setEventFlagState(flagCheckNewCycle,false);
			unsigned int currentCycle = getCurrentCycle();
			unsigned int cycleChange = (getEventFlagRange(flagStartNewCycle,32));
			bool sign = getEventFlagState(flagSignNewCycle);
			
			if (!sign){
				if (currentCycle >= cycleChange){
					setCurrentCycle(currentCycle-cycleChange);
				} else {
					setCurrentCycle(0);
				}
			} else {
				setCurrentCycle(currentCycle+cycleChange);
			}
		}
		
		unsigned int gameCycle = getCurrentCycle();
		if (gameCycle > 6){
			auto [row1, _1] = from::param::ClearCountCorrectParam[7];
			auto [row2, _2] = from::param::ClearCountCorrectParam[107];
			
			#define X(field) \
				row1.field = originalMax.field + (cycleIncrease.field * (gameCycle-7)); \
				row2.field = originalMax.field + (cycleIncrease.field * (gameCycle-7));
			CLEAR_COUNT_CORRECT_PARAM_FIELDS
			#undef X
		}
	}
};

void base() {
	std::string section = "endless cycles";
	readConfig(
		std::forward_as_tuple(fixStandardDamage,section,"fix standard damage"_s),
		std::forward_as_tuple(flagCheckNewCycle,section,"event flag check"_s),
		std::forward_as_tuple(flagSignNewCycle,section,"event flag sign"_s),
		std::forward_as_tuple(flagStartNewCycle,section,"event flag range start"_s)
	);
	
	from::DLSY::wait_for_system(-1);
	from::CS::SoloParamRepository::wait_for_params(-1);
	MH_STATUS status = MH_Initialize();
	Log("MinHook Status: ",MH_StatusToString(status));
	
	//uncap max health
	performPatch(healthCapAob,"49 0f 4e c0","49 8b c0 90",healthCapOffset);
	//uncap 9999 ng cycles
	performPatch(ngLoadCapAob,"48 0f 4e c2","48 8b c2 90",ngLoadCapOffset);
	performPatch(ngNewCapAob,"49 0f 4e c0","49 8b c0 90",ngNewCapOffset);
	performPatch(loopCountAob,"48 0f 4e c2","48 8b c2 90",loopCountOffset);
	performPatch(nextLoopCountAob1,"48 0f 4e ca","48 8b ca 90",nextLoopCountOffset1);
	performPatch(nextLoopCountAob2,"48 0f 4e c2","48 8b c2 90",nextLoopCountOffset2);
	
	//get gameDataMan
	{
		int instructionSize = 7;
		int aobOffset = 3;
		uintptr_t baseAddress = AobScan(gameDataManAob);
		uintptr_t gameDataManOffset = (uintptr_t)*(int*)(baseAddress + aobOffset);
		gameDataManPtr = baseAddress+instructionSize+gameDataManOffset;
	}
	
	if (fixStandardDamage){
		//without this, the damage gets multiplied twice.
		 for (auto [id, row] : from::param::ClearCountCorrectParam) {
			 if (id != 0 && id != 100){
				 row.NeturalAttackRate = 1.0;
			 }
		 }
	}
	
	from::paramdef::CLEAR_COUNT_CORRECT_PARAM_ST cycleIncreaseCounter{};
	for (int i = 2; i < 8; i++) {
		auto [row1, _1] = from::param::ClearCountCorrectParam[i-1];
		auto [row2, _2] = from::param::ClearCountCorrectParam[i];
		
		#define X(field) cycleIncreaseCounter.field += row2.field - row1.field;
		CLEAR_COUNT_CORRECT_PARAM_FIELDS
		#undef X
		
		if (i == 7){
			#define X(field) originalMax.field = row2.field;
			CLEAR_COUNT_CORRECT_PARAM_FIELDS
			#undef X
		}
	}
	{//get average increase per ng cycle
		#define X(field) cycleIncrease.field = (cycleIncreaseCounter.field - 1) / 6;
		CLEAR_COUNT_CORRECT_PARAM_FIELDS
		#undef X
	}
	
	from::unique_ptr<set_ng_cycle_task> ng_cycle_task =
		from::make_unique<set_ng_cycle_task>();
	ng_cycle_task->register_task(from::CS::CSTaskGroup::MenuMan);

	hookCall(getMessage,getMessageAob,getMessageOffset,4,&getMessageOriginal);
	hookCall(getGoods,getGoodsAob,getGoodsOffset,4,&getGoodsOriginal);

	MH_STATUS applyStatus = MH_ApplyQueued();
	Log("Apply Status: ",MH_StatusToString(applyStatus));
	
	Sleep(INFINITE);
}
